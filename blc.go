package main

import "fmt"

func main() {
	fmt.Println(eval(parse(toBuf("00100101"))))
}

func toBuf(s string) buffer {
	buf := make([]bool, len(s))
	for i, r := range s {
		switch r {
		case '0':
			buf[i] = false
		case '1':
			buf[i] = true
		default:
			panic(r)
		}
	}
	return buffer{buf}
}

type lambda struct {
	body any
}

type application struct {
	fn any
	arg any
}

type variable uint

type buffer struct {
	buf []bool
}

func (b *buffer) next() bool {
	r := b.buf[0]
	b.buf = b.buf[1:]
	return r
}

func parse(b buffer) any {
	if b.next() {
		var v variable
		for b.next() {
			v++
		}
		return v

	} else {
		if !b.next() {
			return lambda{parse(b)}
		}
		return application{fn: parse(b), arg: parse(b)}
	}
}

func eval(p any) any {
	return evalEnv(p, nil)
}

func evalEnv(p any, env []any) any {
	switch x := p.(type) {
	case variable:
		return env[len(env) - int(x) - 1]
	case application:
		return evalEnv(x.fn, append(env, x.arg))
	case lambda:
		return stringify(p)
	}
	panic("unreachable")
}

func stringify(p any) string {
	switch x := p.(type) {
	case variable:
		s := "1"
		for i := 0; i < int(x); i++ {
			s += "1"
		}
		return s + "0"
	case application:
		return "01" + stringify(x.fn) + stringify(x.arg)
	case lambda:
		return "00" + stringify(x.body)
	}
	panic("unreachable")
}
