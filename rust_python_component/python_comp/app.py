from rust_comp import Root
#from rust_comp.exports import rust_example_comp
from rust_comp.exports.rust_example_comp import Operation, Expression
from wasmtime import Config, Engine, Store

import sys

def main(expr):
    config = Config()
    engine = Engine(config=config)
    store = Store(engine)
    
    # Instantiate the Root component to access RustExampleComp
    print(f'Wasm component initilazing...\n')
    rust_component = Root(store)
    rust_example_comp_ = rust_component.rust_example_comp()
    
    
    
    ##call the eval function
    result = rust_example_comp_.eval(store, expr)
    print(f'The result from rust component on python: {result}')
    
    
if __name__ == '__main__':
    if len(sys.argv) != 4:
        print(f'usage: app.py <operand1> <operand2> <operation: add | sub | mul | div | pow>')
    else:
        #Create an Expression instance to test 'eval'
        expr = Expression(None, None, None)
        expr.first = int(sys.argv[1])
        expr.second = int(sys.argv[2])
        
        #match operation
        match sys.argv[3]:
            case 'add':
                expr.operation = Operation.ADD
            case 'sub':
                expr.operation = Operation.SUB
            case 'mul':
                expr.operation = Operation.MUL
            case 'div':
                expr.operation = Operation.DIV
            case 'pow':
                expr.operation = Operation.POW
            case _:
                print(f'Please enter a valid operation: add | sub | mul | div | pow')
        
        if(expr.operation is not None):
            main(expr)            
    