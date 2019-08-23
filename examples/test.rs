mod my_test {
  
  

  trait Naming {
    pub name : String;
    pub fn show_name(self) {
      self.name;    }
  }

  #[derive(Serialize, Deserialize)]
  pub  Animal {
    pub name : String;    pub  sound : String;    pub age : Int32;  }

   Cat  Animal {
    playful : Boolean;  }

   Naming  Cat {
  }

   Cat {
    pub fn full_name( self, last_name : String) : Result