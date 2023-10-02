/**
* Observer: Pattern
* - Subject Abstract class
* - Observer Abstract class
*
* - Concrete Subjects
* - Concrete Observers
**/ 
use std::cell::RefCell;
use std::rc::Rc;

trait YouTubeChannalSubject {
  /* subject abstract */
  fn subcribe(&mut self, observer: Rc<dyn SubscriberObserver>);

  fn unscribe(&mut self, observer: Rc<dyn SubscriberObserver>);

  fn notify(&self, msg: Rc<String>);
}

trait SubscriberObserver {
  fn get_id(&self) -> i32;

  fn update(&self, msg: Rc<String>);
}


struct YouTubeChannal {
  subscribers: Vec<Rc<dyn SubscriberObserver>>
}

impl YouTubeChannal {
  fn new() -> YouTubeChannal {
    YouTubeChannal {
      subscribers: vec![]
    }
  }
}

impl YouTubeChannalSubject for YouTubeChannal {
  fn subcribe(&mut self, observer: Rc<dyn SubscriberObserver>) {
    self.subscribers.push(Rc::clone(&observer));
  }

  fn unscribe(&mut self, observer: Rc<dyn SubscriberObserver>) {
    let idx = self.subscribers.iter().position(|x| (*x).get_id() == observer.get_id()).unwrap();
    self.subscribers.remove(idx);
  }

  fn notify(&self, msg: Rc<String>) {
    for observer in self.subscribers.iter() {
      observer.update(Rc::clone(&msg));
    }
  }
}


struct YouTubeAccount {
  id: i32,
  messages: RefCell<Vec<Rc<String>>>
}

impl YouTubeAccount {
  fn new(id: i32) -> YouTubeAccount {
    YouTubeAccount {
      id,
      messages: RefCell::new(vec![])
    }
  }
}

impl SubscriberObserver for YouTubeAccount {
  fn update(&self, msg: Rc<String>) {
    let mut messages = self.messages.borrow_mut();
    messages.push(Rc::clone(&msg));
  }

  fn get_id(&self) -> i32 {
    self.id
  }
}


struct SubYouTubeChannal {
  id: i32,
  messages: RefCell<Vec<Rc<String>>>
}

impl SubYouTubeChannal {
  fn new(id: i32) -> SubYouTubeChannal {
    SubYouTubeChannal {
      id,
      messages: RefCell::new(vec![])
    }
  }
}

impl SubscriberObserver for SubYouTubeChannal {
  fn update(&self, msg: Rc<String>) {
    let mut messages = self.messages.borrow_mut();
    messages.push(Rc::clone(&msg));
  }

  fn get_id(&self) -> i32 {
    self.id
  }
}


fn main() {
  let mut youtube_channal = YouTubeChannal::new();

  let youtube_account = Rc::new(YouTubeAccount::new(1));
  let subyoutube_channal = Rc::new(SubYouTubeChannal::new(2));

  youtube_channal.subcribe(Rc::clone(&youtube_account) as Rc<dyn SubscriberObserver>);
  youtube_channal.subcribe(Rc::clone(&subyoutube_channal) as Rc<dyn SubscriberObserver>);

  youtube_channal.notify(Rc::new(String::from("Hello from my youtube channal!")));

  youtube_channal.unscribe(Rc::clone(&subyoutube_channal) as Rc<dyn SubscriberObserver>);

  let msg = Rc::new(String::from("Hello again from my youtube channal!"));
  youtube_channal.notify(Rc::clone(&msg));

  println!("total messages in youtube_account: {:?}", youtube_account.messages.borrow().len());
  println!("total messages in subyoutube_channal: {:?}", subyoutube_channal.messages.borrow().len());

  println!("\n\n");

  println!("Check RefCount subyoutube_channal: {}", Rc::strong_count(&subyoutube_channal));
  println!("Check RefCount youtube_account: {}", Rc::strong_count(&youtube_account));

  println!("Check RefCount msg: {}", Rc::strong_count(&msg));
}

