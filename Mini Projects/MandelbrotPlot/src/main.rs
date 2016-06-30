use std::str:FromStr;

fn parse_pair<T: FromStr>(s: &str, separator: car) -> Option<(T, T)> 
{
	match s.find(separator)
	{
		None => None,
		     Some(index) => 
		     {
			     match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) 
			     {
				     (Ok(l), Ok(r)) => Some((l,r)),
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
