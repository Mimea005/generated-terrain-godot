use num_traits::{Float, Num};

pub trait Lerp<F>
    where
        F: Float,
        Self: Num
{

    /// Returns a linearly interpolated number between self and other by factor t
    fn lerp(self, other: Self, t: F) -> F;

    /// Returns the factor for the linear interpolation between self and max that returned amount
    fn lerp_inv(self, max: Self, amount: F) -> F;

}

impl <T, F> Lerp<F> for T
where
    T: Num + Copy + Into<F>,
    F: Float
{
    fn lerp(self, other: Self, t: F) -> F {

        (F::one() - t) * self.into() + other.into() * t

    }

    fn lerp_inv(self, max: Self, amount: F) -> F {
        (amount - self.into()) / (max - self).into()
    }
}
