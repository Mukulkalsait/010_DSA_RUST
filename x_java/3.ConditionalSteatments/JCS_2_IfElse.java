
/**
 * JCS_2_IfElse
 */
import java.util.*;

public class JCS_2_IfElse {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Input value of a");
    int a = sc.nextInt();
    System.out.println("Input value of b");
    int b = sc.nextInt();

    if (a > b) {
      System.out.println(" a is greater !!!");
    } else {
      System.out.println(" b is greater !!!");
    }
    sc.close();
  }
}
