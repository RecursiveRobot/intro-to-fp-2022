#region 0. Boilerplate





#endregion

// Partially Applied/Curried Functions
var add2numbers = (int a, int b) => a + b;
var add2numbersPartial = (int a) => (int b) => a + b;
var add2numbersCurried = (int a) => (int b) => add2numbers(a, b);
var plus5 = add2numbersCurried(5);
var twelve = plus5(7);
var fortyTwo = add2numbersPartial(15)(27);

// Composition
var plus12 = (int a) => a + 12;
var mult5 = (int a) => a * 5;
static Func<T1, T3> Compose<T1, T2, T3>(Func<T2, T3> f, Func<T1, T2> g) { return (x => f(g(x))); }
var plus12mult5 = Compose(mult5, plus12);

plus12mult5(1);

// Higher-Order Functions
var characters = new List<string>
{
  "Rusty Venture",
  "Brock Samson",
  "Dean Venture",
  "Hank Venture",
  "Helper Robot",
  "Dr. Orpheus",
  "The Monarch",
  "Dr. Girlfriend"
};

var isDirectFamily = (string name) => name.Contains("Venture");
var directFamily = characters.Where(isDirectFamily);
var doctorCount = characters.Where(n => n.Contains("Dr.")).Count();

var charactersByLastName = characters
  .Select(n => n.Split(" ")[1])
  .GroupBy(n => n)
  .Select(ns => (ns.Key, ns.Count()));

// Pattern Matching
abstract record Shape {}
record Circle(double Radius) : Shape;
record Rectangle(double Width, double Height) : Shape;
record Triangle(double Side1, double Side2, double Side3) : Shape;

static double getArea(Shape shape) =>
  shape switch
  {
    Circle (var r) => Math.PI * Math.Pow(r, 2),
    Rectangle (var w, var h) => w * h,
    Triangle (var a, var b, var c) => ((Func<double>)(() => {
      var s =  (a + b + c) / 2.0;
      return Math.Sqrt(s*(s-a)*(s-b)*(s-c));
    }))(),
    _ => throw new NotImplementedException()
  };

getArea(new Circle(2.5));
getArea(new Rectangle(10, 20));
getArea(new Triangle(2,3,4));








