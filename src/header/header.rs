enum Inner {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    // // If the extension is short enough, store it inline
    // ExtensionInline(InlineExtension),
    // // Otherwise, allocate it
    // ExtensionAllocated(AllocatedExtension),
}
pub struct Method(Inner);

impl Method{
        // / GET
        // pub const GET: Method = Method(Get);

//         /// POST
//         pub const POST: Method = Method(Post);
    
//         /// PUT
//         pub const PUT: Method = Method(Put);
    
//         /// DELETE
//         pub const DELETE: Method = Method(Delete);
    
//         /// HEAD
//         pub const HEAD: Method = Method(Head);
    
//         /// OPTIONS
//         pub const OPTIONS: Method = Method(Options);
    
//         /// CONNECT
//         pub const CONNECT: Method = Method(Connect);
    
//         /// PATCH
//         pub const PATCH: Method = Method(Patch);
    
//         /// TRACE
//         pub const TRACE: Method = Method(Trace);

//             /// Converts a slice of bytes to an HTTP method.
//     pub fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod> {
//         match src.len() {
//             0 => Err(InvalidMethod::new()),
//             3 => match src {
//                 b"GET" => Ok(Method(Get)),
//                 b"PUT" => Ok(Method(Put)),
//                 _ => Method::extension_inline(src),
//             },
//             4 => match src {
//                 b"POST" => Ok(Method(Post)),
//                 b"HEAD" => Ok(Method(Head)),
//                 _ => Method::extension_inline(src),
//             },
//             5 => match src {
//                 b"PATCH" => Ok(Method(Patch)),
//                 b"TRACE" => Ok(Method(Trace)),
//                 _ => Method::extension_inline(src),
//             },
//             6 => match src {
//                 b"DELETE" => Ok(Method(Delete)),
//                 _ => Method::extension_inline(src),
//             },
//             7 => match src {
//                 b"OPTIONS" => Ok(Method(Options)),
//                 b"CONNECT" => Ok(Method(Connect)),
//                 _ => Method::extension_inline(src),
//             },
//             _ => {
//                 if src.len() < InlineExtension::MAX {
//                     Method::extension_inline(src)
//                 } else {
//                     let allocated = AllocatedExtension::new(src)?;

//                     Ok(Method(ExtensionAllocated(allocated)))
//                 }
//             }
//         }
//     }
//       /// Return a &str representation of the HTTP method
//       #[inline]
//       pub fn as_str(&self) -> &str {
//           match self.0 {
//               Options => "OPTIONS",
//               Get => "GET",
//               Post => "POST",
//               Put => "PUT",
//               Delete => "DELETE",
//               Head => "HEAD",
//               Trace => "TRACE",
//               Connect => "CONNECT",
//               Patch => "PATCH",
//             //   ExtensionInline(ref inline) => inline.as_str(),
//             //   ExtensionAllocated(ref allocated) => allocated.as_str(),
//           }
//       }
    
}