use std::{collections::HashMap, time::{Duration, Instant}, u128};

use super::command_parser::SearchType;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
    name: *const str,
    value: Duration
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MetricData {
    pub points: Option<HashMap<SearchType, Vec<Point>>>
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MetricAggregation {
    pub average: u128,
    pub max: u128,
    pub min: u128
}

impl MetricData {
    pub fn aggregate(&mut self) -> HashMap<SearchType, MetricAggregation> {
        let mut max: u128 = u128::MIN;
        let mut min: u128 = u128::MAX;
        let mut average: u128 = 0;

        let mut aggregations = HashMap::new();
        if self.points.is_none() {
            return aggregations;
        }

        let borrowed_data = self.points.as_mut().unwrap();
        for point_for_type in borrowed_data {
            let size_of_data = point_for_type.1.len();
            for point in point_for_type.1 {
                let point_as_nanos = point.value.as_nanos();
                if point_as_nanos > max {
                    max = point_as_nanos;
                }

                if point_as_nanos < min {
                    min = point_as_nanos;
                }

                average += point_as_nanos
            }

            if size_of_data != 0 {
                average /= size_of_data as u128;
            }
            
            aggregations.insert(*point_for_type.0, MetricAggregation{
                average,
                max,
                min
            });
        }

        return aggregations;
    }

    pub fn add_new_metric(&mut self, name: *const str, content: &str, pattern: &str, search_type: SearchType, f: fn(&str, &str, SearchType) -> Option<Vec<usize>>) -> (Option<Vec<usize>>, Point) {
        let start = Instant::now();
        let response = f(content, pattern, search_type);
        let duration = start.elapsed();        

        let point = Point{ name, value: duration};
        if self.points.is_none() {
            self.points = Some(HashMap::new());
        }

        if self.points.as_mut().unwrap().get_key_value(&search_type).is_none() {
            self.points.as_mut().unwrap().insert(search_type, Vec::new());
        }

        let current_points = self.points.as_mut().unwrap().get_mut(&search_type).unwrap();
        current_points.push(point);
        return (response, point);
    }
}