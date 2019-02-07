def map_param(param):
    n, t = param.split(': ')
    return to_snake_case(n), t

import sys

def p(s):
    sys.stdout.write(s)

import re

first_cap_re = re.compile('(.)([A-Z][a-z]+)')
all_cap_re = re.compile('([a-z0-9])([A-Z])')

def to_snake_case(name):
    s1 = first_cap_re.sub(r'\1_\2', name)
    return all_cap_re.sub(r'\1_\2', s1).lower()

param_dict = {
    'c_str': '&str',
    '*mut i32': '&mut i32'
}

def to_rust_param(typ):
    if typ in param_dict:
        return param_dict[typ]
    else:
        if typ[0].isupper():
            return 'impl Into<' + typ + '>'
        else:
            return typ

convert_dict = {
    'c_str': '.as_native()',
    '*mut i32': ' as *mut i32'
}

def to_native_type(name, typ):
    if typ in convert_dict:
        assert typ in param_dict
        return name + convert_dict[typ]
    else:
        if typ[0].isupper():
            return name + '.into()'
        else:
            return name

def generate_function(line):
    fn_name, fn_params, fn_return = line.strip().split('|')
    if fn_params:
        fn_params = fn_params.split(', ')
    else:
        fn_params = []
    fn_params = list(map(map_param, fn_params))
    rust_name = to_snake_case(fn_name)
    assert rust_name.startswith('gui_')
    rust_name = rust_name[4:]

    p('pub fn ')
    p(rust_name)
    p('(')
    for name, typ in fn_params:
        p(name)
        p(': ')
        p(to_rust_param(typ))
        p(', ')
    p(')')
    if fn_return:
        p(' -> ' + fn_return)
    p(' {\n    unsafe { ')
    p(fn_name + '(')
    for name, typ in fn_params:
        p(to_native_type(name, typ))
        # p(': ')
        # p(to_rust_param(typ))
        p(', ')
    p(') }\n')
    p('}\n\n')

if __name__ == "__main__":
    with open(sys.argv[1]) as f:
        # lines = list(f)
        # for line in [lines[2], lines[34], lines[14]]:
        for line in f:
            generate_function(line)
