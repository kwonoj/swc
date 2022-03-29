#![allow(unused)]
use scoped_tls_hkt::scoped_thread_local;
use swc_common::{
    comments::{Comment, Comments, SingleThreadedComments},
    plugin::Serialized,
    BytePos,
};

// TODO: https://docs.rs/scoped-tls-hkt/latest/scoped_tls_hkt/#mutable-higher-kinded-types
#[derive(Copy, Clone)]
pub struct PluginStorage<'cmt> {
    pub inner: Option<&'cmt SingleThreadedComments>,
}

scoped_thread_local!(
    pub static COMMENTS: for<'a> PluginStorage<'a>
);

extern "C" {
    fn __get_leading_comment_len_proxy(bytes_pos: i32) -> u32;
    fn __get_leading_comment_proxy(bytes_ptr: i32, vec_allocated_ptr: i32, vec_allocated_len: i32);
    fn __alloc(size: usize) -> *mut u8;
    fn __free(ptr: *mut u8, size: usize) -> i32;
}

pub struct PluginComments;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct CommentVecContext(pub u32, pub u32);

impl Comments for PluginComments {
    fn add_leading(&self, pos: BytePos, cmt: Comment) {
        unimplemented!("not implemented yet");
    }

    fn add_leading_comments(&self, pos: BytePos, comments: Vec<Comment>) {
        unimplemented!("not implemented yet");
    }

    fn has_leading(&self, pos: BytePos) -> bool {
        unimplemented!("not implemented yet");
    }

    fn move_leading(&self, from: BytePos, to: BytePos) {
        unimplemented!("not implemented yet");
    }

    fn take_leading(&self, pos: BytePos) -> Option<Vec<Comment>> {
        unimplemented!("not implemented yet");
    }

    fn get_leading(&self, pos: BytePos) -> Option<Vec<Comment>> {
        // First, ask host to get the length of comments to allocate
        let leading_comments_len = unsafe {
            __get_leading_comment_len_proxy(pos.0.try_into().expect(""))
                .try_into()
                .expect("")
        };

        if leading_comments_len == 0 {
            None
        } else {
            /*let allocated_result: Vec<Comment> = Vec::with_capacity(leading_comments_len);
            let serialized = Serialized::serialize(&allocated_result).expect("");

            let comments_ref = serialized.as_ref();
            //let ptr = comments_ref.as_ptr() as i32;
            let len = comments_ref.len() as i32;*/

            let ptr = unsafe { __alloc(leading_comments_len) };

            println!("allocated {:#?}", leading_comments_len);
            unsafe {
                __get_leading_comment_proxy(
                    pos.0.try_into().expect(""),
                    ptr as _,
                    leading_comments_len.try_into().expect(""),
                )
            }

            println!("deserializestart");
            let raw_result_bytes = unsafe {
                std::slice::from_raw_parts(ptr as _, leading_comments_len.try_into().expect(""))
            };
            println!("fromraw");
            let result = Serialized::new_for_plugin(
                raw_result_bytes,
                leading_comments_len.try_into().expect(""),
            );
            println!("serialized");
            let value = Serialized::deserialize(&result).expect("");
            println!("deserializedone");

            Some(value)
        }
    }

    fn add_trailing(&self, pos: BytePos, cmt: Comment) {
        unimplemented!("not implemented yet");
    }

    fn add_trailing_comments(&self, pos: BytePos, comments: Vec<Comment>) {
        unimplemented!("not implemented yet");
    }

    fn has_trailing(&self, pos: BytePos) -> bool {
        unimplemented!("not implemented yet");
    }

    fn move_trailing(&self, from: BytePos, to: BytePos) {
        unimplemented!("not implemented yet");
    }

    fn take_trailing(&self, pos: BytePos) -> Option<Vec<Comment>> {
        unimplemented!("not implemented yet");
    }

    fn get_trailing(&self, pos: BytePos) -> Option<Vec<Comment>> {
        unimplemented!("not implemented yet");
    }

    fn add_pure_comment(&self, pos: BytePos) {
        unimplemented!("not implemented yet");
    }

    fn with_leading<F, Ret>(&self, pos: BytePos, f: F) -> Ret
    where
        Self: Sized,
        F: FnOnce(&[Comment]) -> Ret,
    {
        unimplemented!("not implemented yet");
    }

    fn with_trailing<F, Ret>(&self, pos: BytePos, f: F) -> Ret
    where
        Self: Sized,
        F: FnOnce(&[Comment]) -> Ret,
    {
        unimplemented!("not implemented yet");
    }
}
