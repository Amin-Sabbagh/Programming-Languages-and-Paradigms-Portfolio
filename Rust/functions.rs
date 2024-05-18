// functions.rs
use std::thread;
use std::time::Duration;

pub fn loading() {
    println!("\n\n*******************************************************************************************************************************************\n");
    thread::sleep(Duration::from_secs(3));
}

pub fn intro() {
    println!("\n\tIn the desolate kingdom of shadows, a lone hero dwelled in a secluded cottage with an unlikely companion – a radiant goldfish,
    \ta cherished pet that brought a glimmer of warmth to the hero's otherwise despondent existence. The sinister King, ruling with unbridled malice,
    \tdispatched grotesque monsters to levy an unreasonable tax on the hero and ruthlessly ended the life of his beloved aquatic friend. Sworn to vengeance,
    \tthe hero unearthed a hidden chamber containing a mystical sword, Solaceblade, 
    \ta weapon imbued with the power to unveil the weaknesses of its adversaries upon touch. Gripping the hilt, 
    \tthe hero embarked on a perilous journey, fueled by grief and determination, to dismantle the oppressive regime and bring light to the shadow-laden realm.
    \tThe epic tale of a silent protagonist and Solaceblade had just begun, with the fate of an entire kingdom hanging in the balance.\n\n");
    
}