pub struct Allotment {
    pub reference: String,
    pub start_date: String,
    pub end_date: String,
}

impl Allotment {
    pub fn new(reference: String, start_date: String, end_date: String) -> Self {
        Self {
            reference,
            start_date,
            end_date,
        }
    }
}


#[cfg(test)]
mod test {
    use crate::domain::booking::allotment::Allotment;

    #[test]
    fn create_allotment() {
        let reference = "123".to_string();
        let start_date = "123".to_string();
        let end_date = "123".to_string();

        let allotment = Allotment::new(reference, start_date, end_date);

        assert_eq!(allotment.reference, "123");
        assert_eq!(allotment.start_date, "123");
        assert_eq!(allotment.end_date, "123");
    }
}

