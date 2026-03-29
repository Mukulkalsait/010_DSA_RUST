
/**
 * JCS_7_SwitchCalculator
 */
import java.util.*;

public class JCS_7_SwitchCalculator {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    System.out.println("A:");
    int a = sc.nextInt();
    System.out.println("B:");
    int b = sc.nextInt();

    System.out.println("Method:");
    // TODO:
    char method = sc.next().charAt(0);

    switch (method) {
      case '+':
        System.out.println("The sum of " + a + " and " + b + " is " + (a + b));
        break;

      case '-':
        System.out.println("The Subtraction of " + a + " and " + b + " is " + (a - b));
        break;
      case '*':
        System.out.println("The Multiplication of " + a + " and " + b + " is " + (a * b));
        break;
      case '/':
        System.out.println("The Divisino of " + a + " and " + b + " is " + (a / b));
        break;
      case '%':
        System.out.println("The Moduljo of " + a + " and " + b + " is " + (a % b));
        break;
      default:
        System.out.println("inValid Operator is given please give only +,-,*,/,%");
        break;
    }
    sc.close();
  }
}
