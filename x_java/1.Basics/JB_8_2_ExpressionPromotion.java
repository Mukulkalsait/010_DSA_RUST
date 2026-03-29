/**
 * JB_8.2_ExpressionPromotion
 */
// TODO:
// bcb sil fd
//
// WARN: in Expression Promotion:
// if used experssion in small DT (datatypes) for reassigning it will not
// accept.
// given billow the examples....

public class JB_8_2_ExpressionPromotion {

  public static void main(String[] args) {
    byte a = 3;
    int b = a * 2;
    // see the warning its becouse a is now premoted into Int.

    // so we use typecast here
    byte c = (byte) (a * 2);
    System.out.println(c);

  }
}
