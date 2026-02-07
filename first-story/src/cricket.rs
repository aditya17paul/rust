pub fn scoreboard(){
    let player1:&str = "rohit";
    let player2:&str = "Kishan";
    let player3:&str = "virat";

    let runs:i32=49;
    let  balls:i32=29;
    let wicket=1;
    let run1: i32=30;
    let run2: i32=60;
    let run3: i32=100;


    
    println!("==============================");
    println!("      🏏 CRICKET DASHBOARD     ");
    println!("==============================");
    println!("balls:{}",balls);
    println!("runs:{}",runs);
    println!("wickets:{}",wicket);
   println!("Player      Runs");
    println!("------------------------------");
    println!("{:<12} {}", player1, run1);
    println!("{:<12} {}", player2, run2);
    println!("{:<12} {}", player3, run3);
    
    println!("==============================");
}