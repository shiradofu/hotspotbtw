use super::BatteryInfo;

pub struct FS030W {
    pub name: &'static str,
}

impl FS030W {
    pub fn new() -> FS030W {
        FS030W { name: "FS030W" }
    }

    pub async fn get_battery_info(&self) -> Result<BatteryInfo, String> {
        let client = reqwest::Client::new();
        let endpoint = "http://192.168.100.1/cgi-bin/ajax_get.cgi?which_ajax=ajax_get_battery_data";
        let err_mapper = |_| String::from("failed to fetch battery info");

        let resp = client
            .post(endpoint)
            .send()
            .await
            .map_err(err_mapper)?
            .text()
            .await
            .map_err(err_mapper)?;

        self.parse_battery_info_resp(&resp)
    }

    fn parse_battery_info_resp(&self, resp: &str) -> Result<BatteryInfo, String> {
        let resps: Vec<&str> = resp.split(',').collect();

        let is_charging = match resps.first() {
            Some(status) => *status == "ac",
            None => return Err(format!("unexpected resp format: {}", resp)),
        };

        let percentage = match resps.get(1) {
            Some(p) => p.parse::<i8>().unwrap_or(0),
            None => return Err(format!("unexpected resp format: {}", resp)),
        };

        Ok(BatteryInfo {
            is_charging,
            percentage,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_battery_info_resp_ac_with_battery() {
        let fs030w = FS030W::new();
        let resp = "ac,88,err,1";
        let battery_info = fs030w.parse_battery_info_resp(resp).unwrap();
        assert!(battery_info.is_charging);
        assert_eq!(battery_info.percentage, 88);
    }

    #[test]
    fn parse_battery_info_resp_ac_without_battery() {
        let fs030w = FS030W::new();
        let resp = "ac,err,err,err";
        let battery_info = fs030w.parse_battery_info_resp(resp).unwrap();
        assert!(battery_info.is_charging);
        assert_eq!(battery_info.percentage, 0);
    }

    #[test]
    fn parse_battery_info_resp_battery_only() {
        let fs030w = FS030W::new();
        let resp = "4,88,err,-1";
        let battery_info = fs030w.parse_battery_info_resp(resp).unwrap();
        assert!(!battery_info.is_charging);
        assert_eq!(battery_info.percentage, 88);
    }
}
