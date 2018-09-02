macro_rules! ffxiv_enum {
    ($name:ident { $($variant:ident => $str_repr:expr),+$(,)? }) => {
      #[derive(Debug, Serialize)]
      pub enum $name {
        $($variant,)+
      }

      impl $name {
        pub fn parse(s: &str) -> Option<Self> {
          let res = match s {
            $($str_repr => $name::$variant,)+
            _ => return None,
          };
          Some(res)
        }

        pub fn name(&self) -> &str {
          match *self {
            $($name::$variant => $str_repr,)+
          }
        }
      }
    }
}

pub mod character;
pub mod free_company;