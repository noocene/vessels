mod kind;

use synstructure::decl_derive;

decl_derive!([Kind, attributes(kind)] => kind::derive);
