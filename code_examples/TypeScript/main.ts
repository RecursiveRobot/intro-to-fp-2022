//#region 0. Boilerplate
import _ from "lodash";
import R from "ramda";



// #endregion

// Define data
const characters =
[
	"Rusty Venture",
	"Brock Sampson",
	"Dean Venture",
	"Hank Venture",
	"Helper Robot",
	"Dr. Orpheus",
	"The Monarch",
	"Dr. Girlfriend"
];

// Create predicate
const isDirectFamily = (name: string) => name.includes("Venture");

// Filter data
const directFamily = characters.filter(isDirectFamily);

// Aggregate data
const doctorCount = characters.filter(n => n.includes("Dr.")).length;

const charactersByLastName = _.chain(characters)
	.map(n => n.split(" ")[1])
	.groupBy(n => n)
	.map(ns => [ns[0], ns.length] as [string, number]).value();

// Partially Applied/Curried Functions
const add2numbers = (a: number, b: number) => a + b;
const add2numbersPartial = (a: number) => (b: number) => a + b;
const add2numbersCurried = R.curry(add2numbers);
const plus5 = add2numbersCurried(5);
const twelve = plus5(7);
const fortyTwo = add2numbersPartial(15)(27);

// Composition
const plus12 = (a: number) => a + 12;
const mult5 = (a: number) => a * 5;
function compose<T1, T2, T3>(f: (_: T2) => T3, g: (_: T1) => T2) { return (x: T1) => f(g(x)); }
const plus12mult5 = compose(mult5, plus12);

plus12mult5(1)

// Pattern Matching
type Circle = { kind: "Circle", radius: number };
type Rectangle = { kind: "Rectangle", width: number, height: number };
type Triangle = { kind: "Triangle", side1: number, side2: number, side3: number };
type Shape = Circle | Rectangle | Triangle;

function getArea(shape: Shape) {
	switch (shape.kind) {
		case "Circle": 
			const {radius: r} = shape; 
			return r**2;
		case "Rectangle": 
			const {width: w, height: h} = shape; 
			return w * h;
		case "Triangle": 
			const {side1: a, side2: b, side3: c} = shape;
			const s = (a + b + c) / 2;
			return Math.sqrt(s*(s-a)*(s-b)*(s-c));
	}
}

getArea({kind: "Circle", radius: 2.5});
getArea({kind: "Rectangle", width: 10, height: 20});
getArea({kind: "Triangle", side1: 2, side2: 3, side3: 4});





