# Java Predefined Methods

Below is a categorized and alphabetically ordered list of Java predefined methods with brief explanations.

---

## 1. **String Methods**

- `charAt(int index)` = Returns the character at the specified index of a string.
- `compareTo(String anotherString)` = Compares two strings lexicographically.
- `compareToIgnoreCase(String anotherString)` = Compares two strings lexicographically, ignoring case differences.
- `concat(String str)` = Concatenates the specified string to the end of the current string.
- `contains(CharSequence sequence)` = Returns `true` if the string contains the specified sequence.
- `endsWith(String suffix)` = Returns `true` if the string ends with the specified suffix.
- `equals(Object anObject)` = Compares the string with the specified object.
- `equalsIgnoreCase(String anotherString)` = Compares two strings, ignoring case.
- `getBytes()` = Encodes the string into a sequence of bytes using the platform's default charset.
- `indexOf(int ch)` = Returns the index of the first occurrence of the specified character.
- `indexOf(String str)` = Returns the index of the first occurrence of the specified substring.
- `isEmpty()` = Returns `true` if the string is empty.
- `lastIndexOf(int ch)` = Returns the index of the last occurrence of the specified character.
- `lastIndexOf(String str)` = Returns the index of the last occurrence of the specified substring.
- `length()` = Returns the length of the string.
- `replace(char oldChar, char newChar)` = Replaces all occurrences of the specified character in the string.
- `replaceAll(String regex, String replacement)` = Replaces each substring of the string that matches the given regular expression.
- `split(String regex)` = Splits the string into an array of substrings based on the given delimiter.
- `startsWith(String prefix)` = Returns `true` if the string starts with the specified prefix.
- `substring(int beginIndex)` = Returns a new string that is a substring of the original string, starting from `beginIndex`.
- `substring(int beginIndex, int endIndex)` = Returns a new string that is a substring of the original string, starting from `beginIndex` to `endIndex`.
- `toLowerCase()` = Converts all characters in the string to lowercase.
- `toUpperCase()` = Converts all characters in the string to uppercase.
- `trim()` = Removes leading and trailing whitespaces from the string.

---

## 2. **Math Methods**

- `abs(int a)` = Returns the absolute value of the integer argument.
- `abs(double a)` = Returns the absolute value of the double argument.
- `ceil(double a)` = Returns the smallest integer greater than or equal to the argument.
- `floor(double a)` = Returns the largest integer less than or equal to the argument.
- `max(int a, int b)` = Returns the greater of two `int` values.
- `max(double a, double b)` = Returns the greater of two `double` values.
- `min(int a, int b)` = Returns the smaller of two `int` values.
- `min(double a, double b)` = Returns the smaller of two `double` values.
- `pow(double a, double b)` = Returns the value of the first argument raised to the power of the second argument.
- `random()` = Returns a random `double` value between 0.0 and 1.0.
- `round(float a)` = Rounds the `float` value to the nearest integer.
- `sqrt(double a)` = Returns the square root of the argument.
- `toDegrees(double angrad)` = Converts an angle measured in radians to degrees.
- `toRadians(double angdeg)` = Converts an angle measured in degrees to radians.

---

## 3. **Array Methods**

- `Arrays.copyOf(T[] original, int newLength)` = Copies the specified array, truncating or padding with `null` as needed.
- `Arrays.equals(int[] a, int[] b)` = Compares two arrays for equality.
- `Arrays.fill(int[] a, int val)` = Assigns the specified value to every element of the specified array.
- `Arrays.sort(int[] a)` = Sorts the specified array into ascending numerical order.
- `Arrays.toString(int[] a)` = Returns a string representation of the contents of the specified array.
- `Arrays.asList(T... a)` = Converts an array to a `List`.

---

## 4. **Math and Number Conversion Methods**

- `Integer.parseInt(String s)` = Parses the string as a signed decimal integer.
- `Integer.valueOf(String s)` = Returns an `Integer` object holding the value of the specified `String`.
- `Double.parseDouble(String s)` = Parses the string as a `double` value.
- `Double.valueOf(String s)` = Returns a `Double` object holding the value of the specified `String`.
- `Float.parseFloat(String s)` = Parses the string as a `float` value.
- `Float.valueOf(String s)` = Returns a `Float` object holding the value of the specified `String`.

---

## 5. **System Methods**

- `currentTimeMillis()` = Returns the current time in milliseconds since the Unix epoch (January 1, 1970).
- `exit(int status)` = Terminates the Java Virtual Machine with the specified exit status code.
- `gc()` = Runs the garbage collector to reclaim unused memory.
- `getProperty(String key)` = Gets the system property specified by the key.
- `nanoTime()` = Returns the current value of the most precise available system timer in nanoseconds.
- `setProperty(String key, String value)` = Sets the system property specified by the key.

---

## 6. **Object Methods**

- `clone()` = Creates and returns a copy of the object.
- `equals(Object obj)` = Compares this object to the specified object.
- `getClass()` = Returns the runtime class of the object.
- `hashCode()` = Returns a hash code value for the object.
- `toString()` = Returns a string representation of the object.
- `notify()` = Wakes up a single thread that is waiting on this object.
- `notifyAll()` = Wakes up all threads that are waiting on this object.
- `wait()` = Causes the current thread to wait until another thread invokes the `notify()` or `notifyAll()` method.

---

## 7. **Exception Handling Methods**

- `getMessage()` = Returns the detail message string of the exception.
- `printStackTrace()` = Prints the stack trace of the exception.
- `toString()` = Returns a string representation of the exception.

---

## 8. **Collection Framework Methods**

### **List Methods**

- `add(E e)` = Adds the specified element to the list.
- `clear()` = Removes all elements from the list.
- `contains(Object o)` = Returns `true` if the list contains the specified element.
- `get(int index)` = Returns the element at the specified position in the list.
- `indexOf(Object o)` = Returns the index of the first occurrence of the specified element.
- `remove(int index)` = Removes the element at the specified position in the list.
- `size()` = Returns the number of elements in the list.

### **Set Methods**

- `add(E e)` = Adds the specified element to the set.
- `clear()` = Removes all elements from the set.
- `contains(Object o)` = Returns `true` if the set contains the specified element.
- `isEmpty()` = Returns `true` if the set is empty.
- `remove(Object o)` = Removes the specified element from the set.
- `size()` = Returns the number of elements in the set.

### **Map Methods**

- `put(K key, V value)` = Associates the specified value with the specified key in the map.
- `get(Object key)` = Returns the value to which the specified key is mapped.
- `containsKey(Object key)` = Returns `true` if the map contains the specified key.
- `containsValue(Object value)` = Returns `true` if the map contains the specified value.
- `remove(Object key)` = Removes the mapping for the specified key from the map.
- `size()` = Returns the number of key-value mappings in the map.

---

## 9. **Threading Methods**

- `start()` = Starts the thread's execution.
- `sleep(long millis)` = Causes the current thread to sleep for the specified number of milliseconds.
- `join()` = Waits for the thread to die.
- `interrupt()` = Interrupts the thread.
- `isAlive()` = Tests if the thread is alive.

---

## 10. **File I/O Methods**

- `exists()` = Returns `true` if the file or directory exists.
- `createNewFile()` = Creates a new, empty file if it does not already exist.
- `delete()` = Deletes the file or directory.
- `length()` = Returns the size of the file in bytes.
- `renameTo(File dest)` = Renames the file or directory.
- `setReadable(boolean readable)` = Sets the read access permission for the file.

---
