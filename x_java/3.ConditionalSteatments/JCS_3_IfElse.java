
/**
 * JCS_3_IfElse
 */
import java.util.*;

public class JCS_3_IfElse {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Insert value of A =");
    int a = sc.nextInt();
    if (a % 2 == 0) {
      System.out.println("The value of a " + a + "is Even Value.");
    } else {
      System.out.println("The value of a " + a + " is Odd Value.");
    }
    sc.close();
  }
}
