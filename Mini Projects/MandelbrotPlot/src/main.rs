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


