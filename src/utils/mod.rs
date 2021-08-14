


const TOP_EDGE:&str = "\n
/*******************************************************************************
*                                                                              *";

const BOTTOM_EDGE:&str = "
*                                                                              *
*******************************************************************************/
";


pub fn banner(title: &str) -> String {
    let mid = format!("\n*{:^78}*", title);
    format!("{}{}{}", TOP_EDGE, mid, BOTTOM_EDGE)
}
