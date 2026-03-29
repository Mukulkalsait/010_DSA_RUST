
/**
 * JCS_5_ifelse_largeNum
 */
import java.util.*;

public class JCS_5_ifelse_largeNum {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("input 1st value =");
    int a1 = sc.nextInt();
    System.out.println("input 2nd value =");
    int a2 = sc.nextInt();
    System.out.println("input 3rd value =");
    int a3 = sc.nextInt();

    // if (a1 > a2) {
    // if (a1 > a3) {
    // System.out.println("the 1st input " + a1 + " is greatest of all !!!");
    // } else {
    // System.out.println("the 3rd input " + a3 + " is greatest of all !!!");
    // }
    // } else {
    // if (a2 > a3) {
    // System.out.println("the 2nd input " + a2 + " is greatest of all !!!");
    // } else {
    // System.out.println("the 3rd input " + a3 + " is greatest of all !!!");
    // }
    // }

    // FIX: with mutlyple conductions in single bracket we are able make same
    // functionility in much smaller code.
    if (a1 > a2 && a1 > a3) {
      System.out.println("the 1st input " + a1 + " is greatest of all !!!");
    } else if (a2 > a3) {
      System.out.println("the 2nd input " + a2 + " is greatest of all !!!");
    } else {
      System.out.println("the 3rd input " + a3 + " is greatest of all !!!");
    }
    sc.close();
  }
}
