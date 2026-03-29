/**
 * JB_8_ExpressionPromotion
 * INFO: BCB SIL FD
 *
 * WARN: In java Expression needs to be performed between different datatypes ->
 * each aquire different space (bytes) -> so all are converted into something
 * convinent (BIGGER1) ->if possible Int(4bite) but if anything bigger found
 * then that is considerd as final datatype.
 *
 * WARN:
 * Hence
 * Byte, Char, Short -> Int
 * but if anything like Long, Float, Double are present the finial product will
 * be it.
 * NOTE: Boolean in java is special type you cannot mix it with any other
 * datadype at all. thats it
 *
 */

public class JB_8_ExpressionPromotion {

  public static void main(String[] args) {

    char a = 'a';
    char b = 'b';

    System.out.println((int) (a));
    System.out.println((int) (b));
    // the above 2 lines will give intiger form of a && b becouse its stated like
    // that.

    System.out.println(a);
    System.out.println(b);
    // the above 2 lines will print orignal form of a && b.

    System.out.println(b - a);
    // FIX: In an expresion even though the char is stated as char it will converted
    // into intigerhence ,,,,,,,, in EXPRESSION ----BITE ,CHAE && SHORT is converted
    // into INT & then expression is performed.

    char x = 'a';
    byte y = 34; // only 2 char numbers...
    short z = 456;

    int sum = x + y + z; // even NONE of X Y Z are int but becouse its expession its in int and thats
                         // only way to do it.
    System.out.println(sum);

    System.out.println("but we can TypeCast the andwer for being anything like BYTE here ->");
    System.out.println((byte) (x + y + z));
    System.out.println("Warning:this answer aboue might not make any sense. ");

    // FIX: lets try using higher memory datadypes.
    //
    // sum will be taken from abuve
    int p = sum;
    long m = 123;
    float n = 23.49f;
    double o = 23.9495;

    double total = p + m + n + o; // changing the DATATYPE of thsi statement will dive error untill se get the
    System.out.println(
        "_______________________________________________\n This billow is the total of int , long, float & double !!!");
    System.out.println(total);
    // proper one.
    //
    // TODO:
    // with p + m + n + o only double is suported .
    // with p + m + n float & double is suported .
    // with p + m lont, float & double is suported .
    //
    // FIX: Even though long is biger (8bit) than float (4bit) for the sake of
    // pression long is TYPECasted into Folat
    // becouse float has actual values like after "." where long is INTIGER type so
    // it has rather FIX values.
    //
  }
}
