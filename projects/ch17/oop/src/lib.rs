/// 隠蔽化・カプセル化の例
/// add や removeメソッドで使用するupdate_averageがプライベートになっている
/// これで、実装の詳細をユーザー(ライブラリ使用者)から隠蔽できる

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32){
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }

    }
    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) { // この関数が隠蔽されている
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}