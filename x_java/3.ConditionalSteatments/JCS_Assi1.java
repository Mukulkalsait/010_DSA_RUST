
/**
 * JCS_Assi1
 */
import java.util.*;

public class JCS_Assi1 {
  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);
    System.out.println("inter any positive or nigitive number:");
    int num = sc.nextInt();

    String evenOdd = (num % 2 == 0) ? "Even" : "Odd";
    String posNigitie = (num > 0) ? "Positive" : "Nigitive";

    System.out.println("The Number " + num + " is " + evenOdd + " && " + posNigitie + " Number.");
    sc.close();
  }
}
