pub fn star_player(){
    let player_name:&str="virat";
    let runs:i32=99;
    println!("player:{}",player_name);
    println!("runs:{}",runs);
    if runs>=100{
        println!("{} is ⭐ Star Player",player_name);
    }else{
        println!("{} is still playing",player_name);
    }
}