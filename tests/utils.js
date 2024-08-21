
import { expect }                       from 'chai';


export async function expect_reject ( cb, error, message ) {
    let failed                          = false;
    try {
        await cb();
    } catch (err) {
        failed                          = true;
        expect( () => { throw err }     ).to.throw( error, message );
    }
    expect( failed                      ).to.be.true;
}


export function linearSuite ( name, setup_fn, args_fn ) {
    describe( name, function () {
        beforeEach(function () {
            let parent_suite            = this.currentTest.parent;
            if ( parent_suite.tests.some(test => test.state === "failed") )
                this.skip();
            if ( parent_suite.parent?.tests.some(test => test.state === "failed") )
                this.skip();
        });
        setup_fn.call( this, args_fn );
    });
}


export default {
    expect_reject,
    linearSuite,
};
