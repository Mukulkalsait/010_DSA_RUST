import java.util.*;

public class JCS_Assi4 {
  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);

    System.out.println("Inter the Initila num:");
    int initialVal = sc.nextInt();
    int realIV = initialVal;
    System.out.println("Inter the Final num:");
    int finalVal = sc.nextInt();
    int sum = 0;

    if (initialVal < finalVal) {
      while (initialVal <= finalVal) {
        sum = sum + initialVal;
        initialVal++;
      }
      System.out.println("The Sum of all numbers from " + realIV + " to " + finalVal + " is " + sum);
    } else {
      System.err.println("ERROR: Please make sure Initial value is smaller than Final Value. !!!");
    }
    sc.close();
  }
}
