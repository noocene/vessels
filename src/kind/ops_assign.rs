use std::ops::{
    AddAssign as IAddAssign, BitAndAssign as IBitAndAssign, BitOrAssign as IBitOrAssign,
    BitXorAssign as IBitXorAssign, DivAssign as IDivAssign, MulAssign as IMulAssign,
    RemAssign as IRemAssign, ShlAssign as IShlAssign, ShrAssign as IShrAssign,
    SubAssign as ISubAssign,
};

use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    executor::ThreadPool,
    future::BoxFuture,
    SinkExt, StreamExt, TryFutureExt,
};

macro_rules! ops_assign_impl {
    ($($trait:ident::$method:ident with $shim:ident),+) => {$(

        struct $shim<T: Kind>(UnboundedSender<T>);

        impl<T: Kind> $trait<T> for $shim<T> {
            fn $method(&mut self, rhs: T) {
                self.0.start_send(rhs).unwrap();
            }
        }

        impl<T: Kind> Kind for Box<dyn $trait<T> + Send> {
            type ConstructItem = ();
            type ConstructError = ();
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = ForkHandle;
            type DeconstructError = ();
            type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                mut self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    while let Some(handle) = channel.next().await {
                        $trait::$method(self.as_mut(), channel.get_fork(handle).await.unwrap())
                    }
                    Ok(())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                mut channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let (sender, mut receiver): (UnboundedSender<T>, _) = unbounded();
                    ThreadPool::new().unwrap().spawn_ok(async move {
                        while let Some(operand) = receiver.next().await {
                            channel.send(channel.fork(operand).await.unwrap()).unwrap_or_else(|_| panic!()).await
                        }
                    });
                    Ok(Box::new($shim(sender)) as Box<dyn $trait<T> + Send>)
                })
            }
        }
    )+};
}

ops_assign_impl!(
    IAddAssign::add_assign with AddAssignShim,
    IBitAndAssign::bitand_assign with BitAndAssignShim,
    IBitOrAssign::bitor_assign with BitOrAssignShim,
    IBitXorAssign::bitxor_assign with BitXorAssignShim,
    IDivAssign::div_assign with DivAssignShim,
    IMulAssign::mul_assign with MulAssignShim,
    IRemAssign::rem_assign with RemAssignShim,
    IShlAssign::shl_assign with ShlAssignShim,
    IShrAssign::shr_assign with ShrAssignShim,
    ISubAssign::sub_assign with SubAssignShim
);
