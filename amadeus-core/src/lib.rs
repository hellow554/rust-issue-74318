#![feature(never_type)]
#![feature(specialization)]
#![feature(read_initializer)]
#![recursion_limit = "25600"]
#![allow(unused_variables, unused_must_use, incomplete_features)]

macro_rules! impl_par_dist {
	($($body:tt)*) => {
		$($body)*
		const _: () = {
			use crate::impl_par_dist::*;
			#[allow(unused_imports)]
			$($body)*
		};
	}
}
mod impl_par_dist {
    pub use crate::{
        par_pipe::DistributedPipe as ParallelPipe, par_sink::DistributedSink as ParallelSink,
        par_stream::DistributedStream as ParallelStream,
    };
}

macro_rules! impl_par_dist_rename {
	($($body:tt)*) => {
		$($body)*
		rename! { [
			ParallelStream DistributedStream
			ParallelSink DistributedSink
			ParallelPipe DistributedPipe
			FromParallelStream FromDistributedStream
			IntoParallelStream IntoDistributedStream
			ParStream DistStream
			IterParStream IterDistStream
			into_par_stream into_dist_stream
			par_stream dist_stream
		] $($body)* }
	}
}
macro_rules! rename {
	([$($from:ident $to:ident)*] $($body:tt)*) => (rename!(@inner [$] [$($from $to)*] $($body)*););
	(@inner [$d:tt] [$($from:ident $to:ident)*] $($body:tt)*) => (
		macro_rules! __rename {
			$(
				(@munch [$d ($d done:tt)*] $from $d ($d body:tt)*) => (__rename!{@munch [$d ($d done)* $to] $d ($d body)*});
			)*
			(@munch [$d ($d done:tt)*] { $d ($d head:tt)* } $d ($d body:tt)*) => (__rename!{@munch [$d ($d done)* { __rename!{$d ($d head)*} }] $d ($d body)*});
			(@munch [$d ($d done:tt)*] $d head:tt $d ($d body:tt)*) => (__rename!{@munch [$d ($d done)* $d head] $d ($d body)*});
			(@munch [$d ($d done:tt)*]) => ($d ($d done)*);
			(@__rename $d i:ident) => ($d i);
			($d ($d body:tt)*) => (__rename!{@munch [] $d ($d body)*});
		}
		__rename!($($body)*);
	);
}

pub mod into_par_stream;
pub mod par_pipe;
mod par_sink;
pub mod par_stream;
pub mod pool;
pub mod sink;
mod source;
pub mod util;

pub use source::*;
