
pub struct Context<'s>(&'s str);

pub struct Parser<'c ,'s: 'c> {
    context: &'c Context<'s>,
}

impl<'c , 's> Parser<'c ,'s> {
    pub fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

pub fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

pub struct Ref<'a, T>(&'a T);
pub struct LongRef<'long_life, T>(&'long_life T); // この書き方できるのイイね！
pub struct StaticRef<T: 'static>(&'static T);

