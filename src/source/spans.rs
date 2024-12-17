use std::ops::Deref;
use crate::{Sample, Source};
use std::sync::mpsc;
use std::time::Duration;

pub trait Pluggable<S: Sample>: Source<Item = S>
where
    Self::Item: Sample,
{
    fn connect(&mut self, source: Box<dyn Source<Item = S>>);
}

pub struct Spans<S> {
    input: Option<Box<dyn Source<Item = S>>>,
    init: Box<dyn Fn() -> Box<dyn Pluggable<S>>>,
    wrapped: Option<Box<dyn Pluggable<S>>>,
    source_trigger: mpsc::Receiver<Box<dyn Source<Item = S>>>,
}

impl<S: Sample + 'static> Spans<S> {
    pub fn new(
        input: Box<dyn Source<Item = S>>,
        init: Box<dyn Fn() -> Box<dyn Pluggable<S>>>,
    ) -> Self {
        let (tx, rx) = mpsc::channel::<Box<dyn Source<Item = S>>>();
        let mut result = Spans {
            input: None,
            init,
            wrapped: None,
            source_trigger: rx,
        };
        let span_len = result.input.as_ref().unwrap().current_frame_len();
        let mut core = result.init.deref()();
        core.connect(Box::new(SpanSource {
            count: span_len,
            source: Some(input),
            handle_end: Some(move |source| {
                tx.send(source)
                    .expect("source can be returned after span is exhausted")
            }),
        }));
        result.wrapped = Some(core);
        todo!("after receiving source back, make and attach a new span processor");
        result
    }
}

// See TestSource for a reference implementation
struct SpanSource<S, Cb>
where
    S: Sample + 'static,
    Cb: FnOnce(Box<dyn Source<Item = S>>),
{
    count: Option<usize>,
    source: Option<Box<dyn Source<Item = S>>>,
    handle_end: Option<Cb>,
}

impl<S, Cb> SpanSource<S, Cb>
where
    S: Sample,
    Cb: FnOnce(Box<dyn Source<Item = S>>),
{
    fn return_source(&mut self) {
        let cb = self.handle_end.take();
        if let Some(cb) = cb {
            cb(self.source.take().expect("source is handed back only once"));
        }
    }
}

impl<S: Sample, Cb> Source for SpanSource<S, Cb>
where
    Cb: FnOnce(Box<dyn Source<Item = S>>),
{
    fn current_frame_len(&self) -> Option<usize> {
        todo!()
    }

    fn channels(&self) -> u16 {
        todo!()
    }

    fn sample_rate(&self) -> u32 {
        todo!()
    }

    fn total_duration(&self) -> Option<Duration> {
        todo!()
    }
}

impl<S, Cb> Iterator for SpanSource<S, Cb>
where
    S: Sample,
    Cb: FnOnce(Box<dyn Source<Item = S>>),
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.is_none() {
            return None;
        }
        if let Some(sample) = self.source.as_deref_mut().unwrap().next() {
            if let Some(count) = self.count.as_mut() {
                *count -= 1;
                if *count == 0 {
                    self.return_source();
                }
            }
            Some(sample)
        } else {
            self.return_source();
            None
        }
    }
}
