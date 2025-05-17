use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum TempUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug, PartialEq)]
struct Temperature {
    value: f64,
    unit: TempUnit,
}

impl Temperature {
    // 转为摄氏度
    fn to_celsius(&self) -> f64 {
        match self.unit {
            TempUnit::Celsius => self.value,
            TempUnit::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0,
            TempUnit::Kelvin => self.value - 273.15,
        }
    }
    // 转为华氏度
    fn to_fahrenheit(&self) -> f64 {
        match self.unit {
            TempUnit::Celsius => self.value * 9.0 / 5.0 + 32.0,
            TempUnit::Fahrenheit => self.value,
            TempUnit::Kelvin => (self.value - 273.15) * 9.0 / 5.0 + 32.0,
        }
    }
    // 转为开尔文
    fn to_kelvin(&self) -> f64 {
        match self.unit {
            TempUnit::Celsius => self.value + 273.15,
            TempUnit::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0 + 273.15,
            TempUnit::Kelvin => self.value,
        }
    }
}

impl FromStr for TempUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" | "℃" => Ok(TempUnit::Celsius),
            "f" | "℉" => Ok(TempUnit::Fahrenheit),
            "k" => Ok(TempUnit::Kelvin),
            _ => Err(()),
        }
    }
}

impl FromStr for Temperature {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (num_part, unit_part) = s.trim().split_at(
            s.find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
                .unwrap_or(s.len())
        );
        let value: f64 = num_part.trim().parse().map_err(|_| "无法解析数值".to_string())?;
        let unit = if unit_part.trim().is_empty() {
            TempUnit::Celsius // 默认摄氏度
        } else {
            TempUnit::from_str(unit_part.trim()).map_err(|_| "无法识别单位".to_string())?
        };
        Ok(Temperature { value, unit })
    }
}

fn main() {
    println!("请输入温度（如 32F、100C、273.15K，单位可省略，默认为摄氏度）：");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let temp = match Temperature::from_str(&input) {
        Ok(t) => t,
        Err(e) => {
            println!("输入错误: {}", e);
            return;
        }
    };

    println!(
        "原始输入: {:.2} {:?}\n摄氏度: {:.2}℃\n华氏度: {:.2}℉\n开尔文: {:.2}K",
        temp.value,
        temp.unit,
        temp.to_celsius(),
        temp.to_fahrenheit(),
        temp.to_kelvin()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_temperature() {
        assert_eq!(
            Temperature::from_str("100C").unwrap(),
            Temperature { value: 100.0, unit: TempUnit::Celsius }
        );
        assert_eq!(
            Temperature::from_str("32F").unwrap(),
            Temperature { value: 32.0, unit: TempUnit::Fahrenheit }
        );
        assert_eq!(
            Temperature::from_str("273.15K").unwrap(),
            Temperature { value: 273.15, unit: TempUnit::Kelvin }
        );
        assert_eq!(
            Temperature::from_str("0").unwrap(),
            Temperature { value: 0.0, unit: TempUnit::Celsius }
        );
    }

    #[test]
    fn test_to_celsius() {
        let t = Temperature { value: 32.0, unit: TempUnit::Fahrenheit };
        assert!((t.to_celsius() - 0.0).abs() < 1e-6);
        let t = Temperature { value: 273.15, unit: TempUnit::Kelvin };
        assert!((t.to_celsius() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_to_fahrenheit() {
        let t = Temperature { value: 0.0, unit: TempUnit::Celsius };
        assert!((t.to_fahrenheit() - 32.0).abs() < 1e-6);
        let t = Temperature { value: 273.15, unit: TempUnit::Kelvin };
        assert!((t.to_fahrenheit() - 32.0).abs() < 1e-6);
    }

    #[test]
    fn test_to_kelvin() {
        let t = Temperature { value: 0.0, unit: TempUnit::Celsius };
        assert!((t.to_kelvin() - 273.15).abs() < 1e-6);
        let t = Temperature { value: 32.0, unit: TempUnit::Fahrenheit };
        assert!((t.to_kelvin() - 273.15).abs() < 1e-6);
    }

    #[test]
    fn test_invalid_input() {
        assert!(Temperature::from_str("abc").is_err());
        assert!(Temperature::from_str("100X").is_err());
    }
}