/*
   ,,,,,,,,:::;;s3X,:;.;33:,,.;:..;;..:i:,;.;;,..;i,,s;.,;;,.;A53333
   ,,,,,,,,,,::::r3r.i;.;r,;:;i;::ii::;i:,,:i:..:i:.i;.:i:.:Xhhh5AA5
   ,,,,,,,,,,,::::;i,:;;;;iirrrrrrrrrrriiiiii;;;i;,.,,;;,.;A22AXXsir
   ,,,,,,,,,,,,,,,:,:,,;;iiiiiiiiiiiiiiiirriirrrri;::;:,,;rrrrrrrrri
   ,,,,,,,,,,,,,,,,,::,,;iiiiiiiiiiiiiiiiiiiiiriii;;;:..sAAAAAAXXXXX
   ,,,,,,,,,,,,,,,,,:;;,,irirrriiiiiiiiiiiiiirrrii;;;,.:533333333333
   ,,,,,,,,,,,,,,,.,:;ii,:rriirriiiiiiirriirrrrii;;;;,.;5225h5553335
   ,,,,,,,,,,,,,,,,:;s3Ms,;riiiiiiiiiir;:::;irrii;;;;,.rAAAA55553333
   ,,,,,,,,,,,,,,,:;AMMMM;,iriiiiiiiiii:;;;:;rrii;;;:,.s2AAAA2555533
   ,,,,,,,,,,,,,,:;XMMM3Ar,:iriiiiiiiri,:;;::rri;;;;:. s5A5553333335
   ,,,,,,,,,,,,.,;rhMMXs3GA.:rriiiiiiii;:::;irii;;;;:..s555335AXXssX
   :::::,,,,,,.,:;AMMsX#939X.;rriiiiiiirrrrrrrr;,:;;:,.r352Xriiiirri
   ;;;;;;;:::,.::;5Mh;3@S2B#:.;rriiiiiiirrirrii:.:;;:,.i2s;;rX255555
   ;;;;;;;;;;:,:;;XMMsihS9H2Ar,:irriiiiirrrrri;,,;;;:,.:i;s255555555
   i;;;i;;;ii,,;;;iAMMArrsX5MMs,,:irrrrrrrrr;:.,;;;;:,..iA555555522X
   iii;iiiiii,:;;;;irA3MMMMMh5r;;,,:;;ii;;;,,;2r;;;;:,.,25555552si:,
   iiiiiiiiii,:;;;;ii;irsXXsiiiiri;::,,:;::sM9@G;;;;:,.:555552X;,,,,
   iiiiiii;ii,::;;;iiiiiiiiiiiiiiirri;H9B3,i&@@Bi;;::,.,2555A;,,,,::
   iiiiii;iii,:::;;;iiiirrriiiiiiiiri;H@@BHS&@@M;;;::,.:535X;,,,::,,
   iiiii;;iii:,::;;;;iirrrriiiiiiiiii;rH@@@@@@Hi;;::,..s35s:,,,,::::
   ;iiiiiiiii;,:::;;;;;;;iiiiirriiiirriiAhM32si;;;::,.:25X:,,,,:,:::
   iii;iiiiiii,,::;:,;3ri3X:ri;iirrrririiii;;;;;;::,..s22r,.,,::,,,,
   iiiiiiiiiii;,:::;,;A;iMsr&3;As;;irriiiiii;;;:::,..:X22i,.,::::,::
   ii;;iiiiiiii:,::;;,;2;;;;s;iHX;2siiiii;;;;;:::,.,i:r22s:,,,,::,::
   iii;;;;;;::::,.,::::r:3h:sr::,s#hi:;;;;;:::,,..:XA;;252s:,,,:::,:
   iii;,...........,,::::;;:5r:33;;:::,:;;::,,..;s222s;s552X;,,,,,,:
   i;;ii;,...,:::;::,,,,:::::::ii::::::::,,...;X222222s;rA22As;:,,,,
   ;;;;;;;;;:,.,:;iiii;:,,,,,::::::::,,,.,,:;iisA22222As;;sA2AAXXsi:
   ;;irrsX25A:.,:;iiiirri;;;::,,,,.......;irriiirsA222A2Xi:iXA2A2hh5
*/

#![allow(non_snake_case)]

use std::fs;

use MolecularRearranger::{ast::CompilationUnit, generator::CompUnitToAsm, tokeniser::Tokeniser};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2, "You must provide a path to an oxygen file");
    println!("Infusing: {}", args[1]);

    // Tokenise text
    let (tokens, errs) = fs::read_to_string(&args[1])
        .expect("Failed to read file")
        .tokenise();
    println!("{:?}\n", tokens);
    println!("Errors: {:?}\n", errs);

    // Parse AST
    let (compilation_unit, errs) = CompilationUnit::new(tokens);
    println!("{:#?}\n", compilation_unit);
    println!("Errors: {:?}\n", errs);
    let asm = compilation_unit.to_llvm_ir();
    println!("{}\n", asm);
}
