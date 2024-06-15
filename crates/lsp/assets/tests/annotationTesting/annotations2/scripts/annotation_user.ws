@replaceMethod(CR4Player)
function SetActivePoster (poster : W3Poster) {
    //NOTHING!
}

@replaceMethod
function GetWitcherPlayer(): W3PlayerWitcher {
    return NULL;
}

@wrapMethod(CR4Player)
function GetLevel() : int {
    return wrappedMethod() * 2;
}

@addMethod(CR4Player)
function AddedMethod(s: string): void {

}

@addField(CR4Player)
var addedVar: string;


exec function test1() {
    thePlayer.SetActivePoster(NULL);
    GetWitcherPlayer();
    thePlayer.GetLevel();
    thePlayer.AddedMethod("Hello");
    thePlayer.addedVar;
}