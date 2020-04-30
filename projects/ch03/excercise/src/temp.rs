
#[derive(Debug)]
pub struct Temprature {
    temp: f64, //kelvin
}

impl Temprature {
    pub fn from_kelvin(k :f64) -> Temprature{
        Temprature{temp:k}
    }
    pub fn from_celsius(c :f64) -> Temprature{
        Temprature{temp: c + 273.15}

    }
    pub fn from_fahrenheit(f: f64) -> Temprature{
        let f1 :f64 = f + 459.67;
        Temprature{temp: f1*5.0/9.0}
    }

    pub fn as_kelvin(&self) -> f64{
        self.temp
    }

    pub fn as_celsius(&self) -> f64{
        self.temp - 273.15
    }

    pub fn as_fahrenheit(&self) -> f64{
        (self.temp * 9.0 / 5.0) - 459.67
    }
}

#[cfg(test)]
mod tests{
    use super::Temprature;
    #[test]
    fn test_celsius2fahrenheit() {
        let c_0_degree = Temprature::from_celsius(0.0);
        let diff = c_0_degree.as_fahrenheit()-32.00;
        assert!(diff <= 0.001);
    }

    #[test]
    fn test_fahrenheit2celsius() {
    let f50_degree = Temprature::from_fahrenheit(50.0);
    let diff = f50_degree.as_celsius()-10.00;
    assert!(diff <= 0.001);
    }
}