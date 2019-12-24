use futures::future::{BoxFuture, Future, FutureExt};

pub struct Command<T> {
    futures: Vec<BoxFuture<'static, T>>,
}

impl<T> Command<T> {
    pub fn none() -> Self {
        Self {
            futures: Vec::new(),
        }
    }

    pub fn perform<A>(
        future: impl Future<Output = T> + 'static + Send,
        f: impl Fn(T) -> A + 'static + Send,
    ) -> Command<A> {
        Command {
            futures: vec![future.map(f).boxed()],
        }
    }

    pub fn map<A>(mut self, f: impl Fn(T) -> A + 'static + Send + Sync) -> Command<A>
    where
        T: 'static,
    {
        let f = std::sync::Arc::new(f);

        Command {
            futures: self
                .futures
                .drain(..)
                .map(|future| {
                    let f = f.clone();

                    future.map(move |result| f(result)).boxed()
                })
                .collect(),
        }
    }

    pub fn batch(commands: impl Iterator<Item = Command<T>>) -> Self {
        Self {
            futures: commands.flat_map(|command| command.futures).collect(),
        }
    }

    pub fn futures(self) -> Vec<BoxFuture<'static, T>> {
        self.futures
    }
}

impl<T, A> From<A> for Command<T>
where
    A: Future<Output = T> + 'static + Send,
{
    fn from(future: A) -> Self {
        Self {
            futures: vec![future.boxed()],
        }
    }
}

impl<T> std::fmt::Debug for Command<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command").finish()
    }
}
