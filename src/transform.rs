use Result;

/// A scale and an offset that transforms xyz coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// The scale.
    pub scale: f64,
    /// The offset.
    pub offset: f64,
}

impl Transform {
    /// Applies this transform to an i32, returning a float.
    ///
    /// # Examples
    ///
    /// ```
    /// # use las::Transform;
    /// let transform = Transform { scale: 2., offset: 1. };
    /// assert_eq!(3., transform.direct(1));
    /// ```
    pub fn direct(&self, n: i32) -> f64 {
        self.scale * n as f64 + self.offset
    }

    /// Applies the inverse transform, and rounds the result.
    ///
    /// Returns an error if the resultant value can't be representaed as an i32.
    ///
    /// # Examples
    ///
    /// ```
    /// # use las::Transform;
    /// let transform = Transform { scale: 2., offset: 1. };
    /// assert_eq!(1, transform.inverse(2.9).unwrap());
    /// ```
    pub fn inverse(&self, n: f64) -> Result<i32> {
        use Error;
        use std::i32;

        let n = ((n - self.offset) / self.scale).round();
        if n > i32::MAX as f64 {
            Err(Error::Overflow(n))
        } else {
            Ok(n as i32)
        }
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            scale: 0.001,
            offset: 0.,
        }
    }
}