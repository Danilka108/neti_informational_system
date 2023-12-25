use std::pin::Pin;
use std::{future::Future, marker::PhantomData};

type Method<'f, 'm, C, O> =
    Box<dyn FnOnce(C) -> Pin<Box<dyn Future<Output = O> + Send + 'f>> + Send + Sync + 'm>;

pub struct EntityMethod<'f, 'm: 'f, C, O> {
    m: Method<'f, 'm, C, O>,
    _c: PhantomData<C>,
    _o: PhantomData<O>,
}

impl<'f, 'm: 'f, C, O> EntityMethod<'f, 'm, C, O> {
    pub fn new(m: Method<'f, 'm, C, O>) -> Self {
        Self {
            m,
            _c: PhantomData,
            _o: PhantomData,
        }
    }

    // TODO change to &C
    pub async fn exec(self, ctx: C) -> O {
        (self.m)(ctx).await
    }
}

// struct Entity {}

// struct Context<I> {
//     i: I,
// }

// fn test(
//     arg: [u8; {
//         fn inner() -> usize {
//             0
//         };
//         0
//     }],
// ) {
// }
// impl Entity {
//     async fn method<I: ToString>(ctx: Context<I>, name: String, id: &i32) -> Vec<String> {
//         let _a: i32 = 0;
//         vec![name, id.to_string(), ctx.i.to_string()]
//     }

//     fn method_2<
//         'entity_method_future,
//         'entity_method: 'entity_method_future,
//         I: Send + ToString + 'entity_method_future,
//     >(
//         name: Vec<&'entity_method_future str>,
//         id: &'entity_method_future i32,
//     ) -> EntityMethod<'entity_method, 'entity_method_future, Context<I>, Vec<String>> {
//         let m: ::std::boxed::Box<
//             dyn ::std::ops::FnOnce(
//                     Context<I>,
//                 ) -> ::std::pin::Pin<
//                     ::std::boxed::Box<
//                         dyn ::std::future::Future<Output = Vec<String>>
//                             + ::std::marker::Send
//                             + 'entity_method_future,
//                     >,
//                 > + ::std::marker::Send
//                 + ::std::marker::Sync
//                 + 'entity_method,
//         > = ::std::boxed::Box::new(move |ctx| todo!());

//         EntityMethod::new(m)
//         // let m: Box<
//         //     dyn FnOnce(Context<I>) -> Pin<Box<dyn Future<Output = Vec<String>> + Send + 'f>>
//         //         + Send
//         //         + Sync
//         //         + 'm,
//         // > = Box::new(move |ctx: Context<I>| {
//         //     Box::pin(async move {
//         //         let _a: i32 = 0;
//         //         vec![name[0].to_string(), id.to_string(), ctx.i.to_string()]
//         //     })
//         // });

//         // EntityMethod {
//         //     m,
//         //     _c: PhantomData,
//         //     _o: PhantomData,
//         // }
//     }
// }
