// An operator defines some function that will be performed on the data. The data on which operators work are called operands. Consider the following expression −

// 7 + 5 = 12

// Here, the values 7, 5, and 12 are operands, while + and = are operators.

// The major operators in Rust can be classified as −

// Arithmetic
// Bitwise
// Comparison
// Logical
// Bitwise
// Conditional
// Arithmetic Operators
// Assume the values in variables a and b are 10 and 5 respectively.

// Show Examples

// Sr.No	Operator	Description	Example
// 1	+(Addition)	returns the sum of the operands	a+b is 15
// 2	-(Subtraction)	returns the difference of the values	a-b is 5
// 3	* (Multiplication)	returns the product of the values	a*b is 50
// 4	/ (Division)	performs division operation and returns the quotient	a / b is 2
// 5	% (Modulus)	performs division operation and returns the remainder	a % b is 0
// NOTE − The ++ and -- operators are not supported in Rust.

// Relational Operators
// Relational Operators test or define the kind of relationship between two entities. Relational operators are used to compare two or more values. Relational operators return a Boolean value − true or false.

// Assume the value of A is 10 and B is 20.

// Show Examples

// Sr.No	Operator	Description	Example
// 1	>	Greater than	(A > B) is False
// 2	<	Lesser than	(A < B) is True
// 3	>=	Greater than or equal to	(A >= B) is False
// 4	<=	Lesser than or equal to	(A <= B) is True
// 5	==	Equality	(A == B) is fals
// 6	!=	Not equal	(A != B) is True
// Logical Operators
// Logical Operators are used to combine two or more conditions. Logical operators too, return a Boolean value. Assume the value of variable A is 10 and B is 20.

// Show Examples

// Sr.No	Operator	Description	Example
// 1	&& (And)	The operator returns true only if all the expressions specified return true	(A > 10 && B > 10) is False
// 2	||(OR)	The operator returns true if at least one of the expressions specified return true	(A > 10 || B >10) is True
// 3	! (NOT)	The operator returns the inverse of the expression’s result. For E.g.: !(>5) returns false	!(A >10 ) is True
// Bitwise Operators
// Assume variable A = 2 and B = 3.

// Show Examples

// Sr.No	Operator	Description	Example
// 1	& (Bitwise AND)	It performs a Boolean AND operation on each bit of its integer arguments.	(A & B) is 2
// 2	| (BitWise OR)	It performs a Boolean OR operation on each bit of its integer arguments.	(A | B) is 3
// 3	^ (Bitwise XOR)	It performs a Boolean exclusive OR operation on each bit of its integer arguments. Exclusive OR means that either operand one is true or operand two is true, but not both.	(A ^ B) is 1
// 4	! (Bitwise Not)	It is a unary operator and operates by reversing all the bits in the operand.	(!B) is -4
// 5	<< (Left Shift)	It moves all the bits in its first operand to the left by the number of places specified in the second operand. New bits are filled with zeros. Shifting a value left by one position is equivalent to multiplying it by 2, shifting two positions is equivalent to multiplying by 4, and so on.	(A << 1) is 4
// 6	>> (Right Shift)	Binary Right Shift Operator. The left operand’s value is moved right by the number of bits specified by the right operand.	(A >> 1) is 1
// 7	>>> (Right shift with Zero)	This operator is just like the >> operator, except that the bits shifted to the left are always zero.	(A >>> 1) is 1

fn main(){
    let a:i32 = 1;
    let b:i32 = 2;
    if a < b {
        println!("{}", true)
    } else {
        println!("{}", false);
    }
}