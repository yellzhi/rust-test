mod blog;

pub struct AveragedCollection{
    list : Vec<i32>,
    average : f64,
}

impl AveragedCollection{
    pub fn add(&mut self, value: i32){
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) ->Option<i32>{
        let ret = self.list.pop();
        match ret{
            Some(v)=>{
                self.update_average();
                Some(v)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64{
        return self.average;
    }

    pub fn update_average(&mut self){
        let total :i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64
    }
}


//  继承
pub trait Draw{
    fn draw(&self);
}

pub struct Button{
    pub width : i32,
    pub height: i32,
    pub label : String,
}
impl Draw for Button{
    fn draw(&self) {
       println!("Button: {}", self.label)
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox: {:?}", self.options)
    }
}

pub struct Screen{
    pub commponents : Vec<Box<dyn Draw>>,
}
impl Screen{
    pub fn run(&self){
        for com in self.commponents.iter(){
            com.draw();
        }
    }
}