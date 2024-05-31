/* 
 * Warning: this script is a part of native content directory ditributed with WIDE.
 * It includes types accessible from scripts that are not however explicitly declared anywhere.
 * Their reconstructed definition may not be accurate.
 */


// core/array.ws

/*array< T >
{
	// Element access
	operator[int index] : T;

	// Clear array
	function Clear();

	// Get array size
	function Size() : int;
	
	// Add element at the end of array
	function PushBack( element : T );
	
	// Remove element at the end of array
	function PopBack() : T;

	// Resize array
	function Resize( newSize : int );
	
	// Remove given element, returns false if not found
	function Remove( element : T ) : bool;
	
	// Does array contain element?
	function Contains( element : T ) : bool;

	// Find first element, returns -1 if not found
	function FindFirst( element : T ) : int;

	// Find last element, returns -1 if not found
	function FindLast( element : T ) : int;
	
	// Add space to array, returns new size
	function Grow( numElements : int ) : int;
	
	// Erase place in array
	function Erase( index : int );
	
	// Insert item at given position
	function Insert( index : int, element : T );
	
	// Get last element
	function Last() : T;
};*/