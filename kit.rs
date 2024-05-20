# Faddy's Kit

/ ?# mkdir -p .FaddysKit ; ln -fs ~/FaddysStudio/kit/percussive/*.wav .FaddysKit

/ ?# -1 -2 cd .FaddysKit ; npm i @faddys/scenarist

?# cat - > .FaddysKit/index.mjs

+==
import Scenarist from '@faddys/scenarist';
import { parse } from 'node:path';

const { dir } = parse ( new URL ( import .meta .url ) .pathname );

try {

await Scenarist ( new class Kit {

[ '$-n' ] ( $, value, ... argv ) {

this .name = value; return $ ( ... argv );

}

[ '$-p' ] ( $, value, ... argv ) {

this .path = `${ process .cwd () }/${ value }`; return $ ( ... argv );

}

async $_producer ( $ ) {

await $ ( ... process .argv .slice ( 2 ) );

const kit = this;

if ( ! kit .name )
throw 'Kit name cannot be undefined';

if ( ! kit .path )
throw 'Path to audio files to be added to the kit cannot be undefined';

console .log ( [

`cd ${ dir }`,
`mkdir -p ${ kit .name }`,
`ln -fs ${ kit .path }/*.wav ${ kit .name }`

] .join ( ' ; ' ) );

}

} );

} catch ( issue ) {	

console .error ( issue );

}
-==

?# $ node .FaddysKit/index.mjs > .FaddysKit/index.sh

?# bash .FaddysKit/index.sh
