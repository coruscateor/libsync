
pub trait ItemUpdater<T>
{

    //type TheType = T;

    fn init() -> T;

    fn update(item: &mut T);
    
}
