use derive::value;
use erased_serde::Serialize as ErasedSerialize;
use failure::Error;
use futures::{
    future::{empty, ok},
    lazy,
    sink::With,
    stream::{Map, SplitSink, SplitStream},
    sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    Future as IFuture, Poll, Sink, StartSend, Stream,
};
use serde::{
    de::{DeserializeOwned, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Serialize, Serializer,
};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ffi::{CString, OsString},
    fmt,
    marker::PhantomData,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::Deref,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, SystemTime},
};

pub struct Item {
    ty: TypeId,
    func: DeserializeFn,
}

impl Item {
    fn new(ty: TypeId, func: DeserializeFn) -> Self {
        Item { ty, func }
    }
}

type DeserializeFn =
    fn(&mut dyn erased_serde::Deserializer) -> erased_serde::Result<Box<dyn SerdeAny>>;

inventory::collect!(Item);

#[derive(Serialize, Deserialize)]
pub struct ForkRef(u64);

pub trait Fork: Send + 'static {
    fn fork<V: Value>(&self, value: V) -> ForkRef;
    fn get_fork<V: Value>(
        &self,
        fork_ref: ForkRef,
    ) -> Box<dyn IFuture<Item = V, Error = ()> + Send + 'static>;
}

pub trait Channel<
    I: Serialize + DeserializeOwned + Send + 'static,
    O: Serialize + DeserializeOwned + Send + 'static,
>: Stream<Item = I, Error = ()> + Sink<SinkItem = O, SinkError = ()> + Fork
{
    type ForkFactory: Fork;

    fn split_factory(&self) -> Self::ForkFactory;
}

pub trait Target:
    Stream<Item = <Self as Target>::Item> + Sink<SinkItem = <Self as Target>::Item>
{
    type Error;
    type Item;

    fn new_with<V: Value>(
        value: V,
    ) -> Box<dyn IFuture<Item = Self, Error = <Self as Target>::Error> + Send + 'static>;

    fn new<
        V: Value,
        C: Stream<Item = <Self as Target>::Item> + Sink<SinkItem = <Self as Target>::Item> + 'static,
    >(
        item: C,
    ) -> Box<dyn IFuture<Item = V, Error = <Self as Target>::Error> + Send + 'static>;
}

pub trait Value: Send + 'static {
    type ConstructItem: Serialize + DeserializeOwned + Send + 'static;
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Box<dyn IFuture<Item = Self, Error = Error> + Send + 'static>
    where
        Self: Sized;

    type DeconstructItem: Serialize + DeserializeOwned + Send + 'static;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Box<dyn IFuture<Item = (), Error = ()> + Send + 'static>;

    #[doc(hidden)]
    const DO_NOT_IMPLEMENT_THIS_TRAIT_MANUALLY: ();

    fn stream<T: Target>(
        self,
    ) -> Box<dyn IFuture<Item = T, Error = <T as Target>::Error> + Send + 'static>
    where
        Self: Sized,
    {
        T::new_with(self)
    }
}

#[value]
impl Value for () {
    type ConstructItem = ();
    type DeconstructItem = ();

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        _: C,
    ) -> Box<dyn IFuture<Item = (), Error = ()> + Send + 'static> {
        Box::new(ok(()))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        _: C,
    ) -> Box<dyn IFuture<Item = Self, Error = Error> + Send + 'static> {
        Box::new(ok(()))
    }
}

#[value]
impl<T: Send + 'static> Value for PhantomData<T> {
    type ConstructItem = ();
    type DeconstructItem = ();

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        _: C,
    ) -> Box<dyn IFuture<Item = (), Error = ()> + Send + 'static> {
        Box::new(ok(()))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        _: C,
    ) -> Box<dyn IFuture<Item = Self, Error = Error> + Send + 'static> {
        Box::new(ok(PhantomData))
    }
}

macro_rules! primitive_impl {
    ($($ty:ident)+) => {$(
        #[value]
        impl Value for $ty {
            type ConstructItem = $ty;
            type DeconstructItem = ();

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                channel: C,
            ) -> Box<dyn IFuture<Item = (), Error = ()> + Send + 'static> {
                Box::new(channel.send(self).then(|_| Ok(())))
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Box<dyn IFuture<Item = Self, Error = Error> + Send + 'static>
            where
                Self: Sized,
            {
                Box::new(
                    channel
                        .into_future()
                        .map_err(|_| failure::err_msg("test"))
                        .map(|v| v.0.unwrap()),
                )
            }
        }
    )+};
}

primitive_impl!(bool isize i8 i16 i32 i64 usize u8 u16 u32 u64 f32 f64 char CString String Ipv4Addr SocketAddrV4 SocketAddrV6 SocketAddr SystemTime OsString Ipv6Addr Duration NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroUsize NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroIsize);

pub struct Serde<T: Serialize + DeserializeOwned + Send + 'static>(T);

impl<T> From<T> for Serde<T>
where
    T: Serialize + DeserializeOwned + Send + 'static,
{
    fn from(input: T) -> Self {
        Serde(input)
    }
}

impl<T: Serialize + DeserializeOwned + Send + 'static> Deref for Serde<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*#[value]
impl<T> Value for Serde<T>
where
    T: Serialize + DeserializeOwned + Send + 'static,
{
    type ConstructItem = T;
    type DeconstructItem = ();
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Box<dyn IFuture<Item = (), Error = ()> + Send + 'static> {
        Box::new(channel.send(self.0).then(|_| Ok(())))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Box<dyn IFuture<Item = Self, Error = Error> + Send + 'static>
    where
        Self: Sized,
    {
        Box::new(
            channel
                .into_future()
                .map_err(|_| failure::err_msg("test"))
                .map(|v| Serde(v.0.unwrap())),
        )
    }
}*/

pub struct Future<T, E>(Box<dyn IFuture<Item = T, Error = E> + Send + 'static>)
where
    T: Value,
    E: Value;

impl<T: Value, E: Value> Deref for Future<T, E> {
    type Target = Box<dyn IFuture<Item = T, Error = E> + Send + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F> From<F> for Future<F::Item, F::Error>
where
    F: IFuture + Send + 'static,
    F::Error: Value,
    F::Item: Value,
{
    fn from(input: F) -> Self {
        Future(Box::new(input))
    }
}

#[derive(Serialize, Deserialize)]
pub enum FResult {
    Ok(ForkRef),
    Err(ForkRef),
}

#[value]
impl<T, E> Value for Future<T, E>
where
    T: Value,
    E: Value,
{
    type ConstructItem = FResult;
    type DeconstructItem = ();
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Box<dyn IFuture<Item = (), Error = ()> + Send + 'static> {
        Box::new(self.0.then(|v| {
            let fork_factory = channel.split_factory();
            channel
                .send(match v {
                    Ok(v) => FResult::Ok(fork_factory.fork(v)),
                    Err(v) => FResult::Err(fork_factory.fork(v)),
                })
                .then(|_| Ok(()))
        }))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Box<dyn IFuture<Item = Self, Error = Error> + Send + 'static>
    where
        Self: Sized,
    {
        Box::new(channel.into_future().then(|v| match v {
            Ok(v) => ok(match v.0.unwrap() {
                FResult::Ok(r) => Future::<T, E>::from(v.1.get_fork::<T>(r).map_err(|_| panic!())),
                FResult::Err(r) => {
                    Future::<T, E>::from(v.1.get_fork::<E>(r).then(|v| Err(v.unwrap())))
                }
            }),
            _ => panic!("lol"),
        }))
    }
}

struct IdChannelContextState {
    channel_types: HashMap<u32, (TypeId, TypeId)>,
    unused_indices: Vec<u32>,
    next_index: u32,
}

pub trait Format {
    type Representation;

    fn serialize<T: Serialize>(item: T) -> Self::Representation
    where
        Self: Sized;
    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> T::Value
    where
        Self: Sized;
}

pub trait ApplyEncode<'de>:
    Sized + UniformStreamSink<<Self as Context<'de>>::Item> + Context<'de>
{
    fn encode<F: Format + Encode<'de, Self>>(self) -> <F as Encode<'de, Self>>::Output;
}

impl<'de, T> ApplyEncode<'de> for T
where
    T: UniformStreamSink<<Self as Context<'de>>::Item> + Context<'de>,
{
    fn encode<F: Format + Encode<'de, Self>>(self) -> <F as Encode<'de, Self>>::Output {
        <F as Encode<_>>::encode(self)
    }
}

pub trait ApplyDecode<'de> {
    fn decode<F: Format + Decode<'de, Self>>(self) -> <F as Decode<'de, Self>>::Output
    where
        Self: Sized + UniformStreamSink<<F as Format>::Representation> + Context<'de>;
}

impl<'de, T> ApplyDecode<'de> for T {
    fn decode<F: Format + Decode<'de, Self>>(self) -> <F as Decode<'de, Self>>::Output
    where
        Self: Sized + UniformStreamSink<<F as Format>::Representation> + Context<'de>,
    {
        <F as Decode<_>>::decode(self)
    }
}

pub trait Decode<'de, C: UniformStreamSink<<Self as Format>::Representation> + Context<'de>>:
    Format
{
    type Output: Stream<Item = <C as Context<'de>>::Item>
        + Sink<SinkItem = <C as Context<'de>>::Item>;

    fn decode(input: C) -> Self::Output;
}

pub trait Encode<'de, C: UniformStreamSink<<C as Context<'de>>::Item> + Context<'de>>:
    Format
{
    type Output: Stream<Item = <Self as Format>::Representation>
        + Sink<SinkItem = <Self as Format>::Representation>;

    fn encode(input: C) -> Self::Output;
}

impl<
        'de,
        T: Format + 'static,
        C: UniformStreamSink<<Self as Format>::Representation> + Context<'de> + 'static + Send,
    > Decode<'de, C> for T
where
    Self::Representation: Send,
{
    type Output = StreamSink<
        Box<dyn Stream<Item = <C as Context<'de>>::Item, Error = ()> + Send>,
        Box<dyn Sink<SinkItem = <C as Context<'de>>::Item, SinkError = ()> + Send>,
    >;

    fn decode(input: C) -> Self::Output {
        let ctx = input.context();
        let (sink, stream) = input.split();
        StreamSink(
            Box::new(
                stream
                    .map_err(|_| ())
                    .map(move |data| <Self as Format>::deserialize(data, ctx.clone())),
            ),
            Box::new(
                sink.sink_map_err(|_| ())
                    .with(|data| Ok(<Self as Format>::serialize(data))),
            ),
        )
    }
}

impl<
        'de,
        T: Format + 'static,
        C: UniformStreamSink<<C as Context<'de>>::Item> + Context<'de> + 'static + Send + Sized,
    > Encode<'de, C> for T
where
    T::Representation: Send,
    <C as Context<'de>>::Item: Send,
{
    type Output = StreamSink<
        Box<dyn Stream<Item = <Self as Format>::Representation, Error = ()> + Send>,
        Box<dyn Sink<SinkItem = <Self as Format>::Representation, SinkError = ()> + Send>,
    >;

    fn encode(input: C) -> Self::Output {
        let ctx = input.context();
        let (sink, stream) = input.split();
        StreamSink(
            Box::new(stream.map_err(|_| ()).map(<Self as Format>::serialize)),
            Box::new(
                sink.sink_map_err(|_| ())
                    .with(move |data| Ok(<Self as Format>::deserialize(data, ctx.clone()))),
            ),
        )
    }
}

pub trait UniformStreamSink<T>: Sink<SinkItem = T> + Stream<Item = T> {}

impl<T, U> UniformStreamSink<T> for U where U: Sink<SinkItem = T> + Stream<Item = T> {}

pub trait Context<'de> {
    type Item: Serialize + 'static;
    type Target: DeserializeSeed<'de, Value = Self::Item> + Clone + Send + 'static;

    fn context(&self) -> Self::Target;
}

pub struct JSON;

impl Format for JSON {
    type Representation = String;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        serde_json::to_string(&item).unwrap()
    }

    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> T::Value {
        let mut deserializer = serde_json::Deserializer::from_reader(item.as_bytes());
        context.deserialize(&mut deserializer).unwrap()
    }
}

pub struct CBOR {}

impl Format for CBOR {
    type Representation = Vec<u8>;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        serde_cbor::to_vec(&item).unwrap()
    }

    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> T::Value {
        let mut deserializer = serde_cbor::Deserializer::from_reader(item.as_slice());
        context.deserialize(&mut deserializer).unwrap()
    }
}

pub struct AsBytes<T: Format>(PhantomData<T>);

impl<F: Format<Representation = String>> Format for AsBytes<F> {
    type Representation = Vec<u8>;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        F::serialize(&item).as_bytes().to_owned()
    }

    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> T::Value {
        F::deserialize(String::from_utf8(item).unwrap(), context)
    }
}

/*pub trait Transport<
    'de,
    T: Target + Stream<Item = F::Representation> + Sink<SinkItem = F::Representation>,
    F: Format,
>
{
}*/

pub struct StdoutTransport {}

#[derive(Clone)]
pub struct IdChannelContext {
    state: Arc<Mutex<IdChannelContextState>>,
}

impl IdChannelContext {
    fn get(&self, channel: &'_ u32) -> Option<(TypeId, TypeId)> {
        self.state
            .lock()
            .unwrap()
            .channel_types
            .get(channel)
            .map(|c| *c)
    }

    fn add(&self, construct: TypeId, deconstruct: TypeId) -> u32 {
        let mut state = self.state.lock().unwrap();

        if let Some(id) = state.unused_indices.pop() {
            state.channel_types.insert(id, (construct, deconstruct));
            id
        } else {
            let id = state.next_index;
            state.next_index += 1;
            state.channel_types.insert(id, (construct, deconstruct));
            id
        }
    }
}

pub struct IdChannel {
    out_channel: Box<dyn Stream<Item = ChannelItem, Error = ()> + Sync + Send>,
    context: IdChannelContext,
}

pub trait IdChannelFormats {}

impl<T> IdChannelFormats for T where T: Stream<Item = ChannelItem, Error = ()> {}

impl Stream for IdChannel {
    type Item = ChannelItem;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.out_channel.poll()
    }
}

pub trait SerdeAny: erased_serde::Serialize + Any + Send {}

serialize_trait_object!(SerdeAny);

impl<T: ?Sized> SerdeAny for T where T: ErasedSerialize + Any + Send {}

pub struct ChannelItem(u32, Box<dyn SerdeAny>);

impl Serialize for ChannelItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("channel", &self.0)?;
            map.serialize_entry("data", self.1.as_ref())?;
            map.end()
        } else {
            let mut seq = serializer.serialize_seq(Some(2))?;
            seq.serialize_element(&self.0)?;
            seq.serialize_element(self.1.as_ref())?;
            seq.end()
        }
    }
}

struct ItemVisitor(IdChannelContext);

impl<'de> Visitor<'de> for ItemVisitor {
    type Value = ChannelItem;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a channel item")
    }

    /*fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
    }*/

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut channel: Option<u32> = None;
        let mut data = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_ref() {
                "channel" => {
                    if channel.is_some() {
                        return Err(serde::de::Error::duplicate_field("channel"));
                    }
                    channel = Some(map.next_value()?);
                }
                "data" => {
                    if data.is_some() {
                        return Err(serde::de::Error::duplicate_field("data"));
                    }
                    data = Some(map.next_value_seed(Id(channel.unwrap(), &mut self.0))?);
                }
                _ => panic!(),
            }
        }
        let channel = channel.ok_or_else(|| serde::de::Error::missing_field("channel"))?;
        let data = data.ok_or_else(|| serde::de::Error::missing_field("data"))?;
        Ok(ChannelItem(channel, data))
    }
}

struct Id<'a>(u32, &'a mut IdChannelContext);

impl<'de, 'a> DeserializeSeed<'de> for Id<'a> {
    type Value = Box<dyn SerdeAny>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ty = self.1.get(&self.0).unwrap();
        let deserializer = &mut erased_serde::Deserializer::erase(deserializer)
            as &mut dyn erased_serde::Deserializer;
        (inventory::iter::<Item>
            .into_iter()
            .find(|item| item.ty == ty.0)
            .unwrap()
            .func)(deserializer)
        .map_err(|_| panic!())
    }
}

impl<'de> DeserializeSeed<'de> for IdChannelContext {
    type Value = ChannelItem;

    fn deserialize<D>(self, deserializer: D) -> Result<ChannelItem, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        let deserializer = &mut erased_serde::Deserializer::erase(deserializer)
            as &mut dyn erased_serde::Deserializer;
        if human_readable {
            deserializer
                .deserialize_map(ItemVisitor(self))
                .map_err(|e| {
                    println!("{:?}", e);
                    panic!();
                })
        } else {
            deserializer
                .deserialize_seq(ItemVisitor(self))
                .map_err(|e| {
                    println!("{:?}", e);
                    panic!();
                })
        }
    }
}

impl<'de> DeserializeSeed<'de> for IdChannel {
    type Value = ChannelItem;

    fn deserialize<D>(self, deserializer: D) -> Result<ChannelItem, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserializer = &mut erased_serde::Deserializer::erase(deserializer)
            as &mut dyn erased_serde::Deserializer;
        if deserializer.is_human_readable() {
            deserializer
                .deserialize_map(ItemVisitor(self.context))
                .map_err(|e| {
                    println!("{:?}", e);
                    panic!();
                })
        } else {
            deserializer
                .deserialize_seq(ItemVisitor(self.context))
                .map_err(|e| {
                    println!("{:?}", e);
                    panic!();
                })
        }
    }
}

impl Sink for IdChannel {
    type SinkItem = ChannelItem;
    type SinkError = ();

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        Err(())
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Err(())
    }
}

impl<'de> Context<'de> for IdChannel {
    type Item = ChannelItem;
    type Target = IdChannelContext;

    fn context(&self) -> Self::Target {
        self.context.clone()
    }
}

impl Target for IdChannel {
    type Error = ();
    type Item = ChannelItem;

    fn new_with<V: Value>(
        value: V,
    ) -> Box<dyn IFuture<Item = Self, Error = <Self as Target>::Error> + Send + 'static> {
        Box::new(
            IdChannelFork::new_with(value).and_then(|(sender, receiver)| {
                let mut channel_types = HashMap::new();

                channel_types.insert(
                    0,
                    (
                        TypeId::of::<V::ConstructItem>(),
                        TypeId::of::<V::DeconstructItem>(),
                    ),
                );

                ok(IdChannel {
                    out_channel: Box::new(
                        receiver.map(move |v| ChannelItem(0, Box::new(v) as Box<dyn SerdeAny>)),
                    ),
                    context: IdChannelContext {
                        state: Arc::new(Mutex::new(IdChannelContextState {
                            channel_types,
                            next_index: 1,
                            unused_indices: vec![],
                        })),
                    },
                })
            }),
        )
    }

    fn new<
        V: Value,
        C: Stream<Item = <Self as Target>::Item> + Sink<SinkItem = <Self as Target>::Item> + 'static,
    >(
        input: C,
    ) -> Box<dyn IFuture<Item = V, Error = <Self as Target>::Error> + Send + 'static> {
        Box::new(IdChannelFork::deconstruct(input))
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > Fork for IdChannelFork<I, O>
{
    fn fork<V: Value>(&self, value: V) -> ForkRef {
        ForkRef(0)
    }
    fn get_fork<V: Value>(
        &self,
        fork_ref: ForkRef,
    ) -> Box<dyn IFuture<Item = V, Error = ()> + Send + 'static> {
        Box::new(empty())
    }
}

pub struct IdChannelFork<
    I: Serialize + DeserializeOwned + Send + 'static,
    O: Serialize + DeserializeOwned + Send + 'static,
> {
    i: UnboundedReceiver<I>,
    o: UnboundedSender<O>,
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > Stream for IdChannelFork<I, O>
{
    type Item = I;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.i.poll().map_err(|_| ())
    }
}

pub struct StreamSink<T: Stream, U: Sink>(T, U);

impl<T: Stream, U: Sink> Sink for StreamSink<T, U> {
    type SinkItem = U::SinkItem;
    type SinkError = U::SinkError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.1.start_send(item)
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.1.poll_complete()
    }
}

impl<T: Stream, U: Sink> Stream for StreamSink<T, U> {
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.poll()
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > IdChannelFork<I, O>
{
    fn new_with<V: Value<DeconstructItem = I, ConstructItem = O>>(
        value: V,
    ) -> impl IFuture<Item = (UnboundedSender<I>, UnboundedReceiver<O>), Error = ()> {
        let (o, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
        let (oi, i): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
        lazy(move || {
            tokio::spawn(value.deconstruct(IdChannelFork { o: oi, i: oo }));
            ok((o, i))
        })
    }

    fn deconstruct<
        V: Value<DeconstructItem = I, ConstructItem = O>,
        C: Stream<Item = ChannelItem> + Sink<SinkItem = ChannelItem>,
    >(
        input: C,
    ) -> impl IFuture<Item = V, Error = ()> {
        lazy(move || {
            let _ = ();
            Err(())
        })
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > Sink for IdChannelFork<I, O>
{
    type SinkItem = O;
    type SinkError = ();

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.o.start_send(item).map_err(|_| ())
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.o.poll_complete().map_err(|_| ())
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > Channel<I, O> for IdChannelFork<I, O>
{
    type ForkFactory = IdChannelFork<I, O>;

    fn split_factory(&self) -> Self::ForkFactory {
        panic!()
    }
}
