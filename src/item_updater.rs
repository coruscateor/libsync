
pub trait ItemUpdater<T>
{

    fn init() -> T;

    fn update(item: &mut T);
    
}