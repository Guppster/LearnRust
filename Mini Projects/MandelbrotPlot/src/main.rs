use std::str:FromStr;

//<T: FromStr> is "For any type T that implements the FromStr trait"
//This allows us to use <u32>, <f64>, etc
//This method requires a string and a seperator and returns a option pair of type T 
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> 
{
	//First check to see that the seperator exists in the string
	match s.find(separator)
	{
		//if it doesn't return none
		None => None,
		
		//If it does exist, capture the index from the return of find()
		Some(index) => 
		{
			//This statement splits the string at the seperator and parses both sides from string to T
			match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) 
			{
				//if the above match expression is successful return the option pair
				(Ok(l), Ok(r)) => Some((l,r)),
				
				//if it is anything else return None.	
				_ => None
			}
		}
	}
}

#[test}
fn test_parse_pair()
{
	assert_eq!(parse_pair::<i32>("",        ','), None);
	assert_eq!(parse_pair::<i32>("10,",     ','), None);
	assert_eq!(parse_pair::<i32>(",10",     ','), None);
	assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
	assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
	assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
	assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}
