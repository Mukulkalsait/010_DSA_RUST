import java.util.*;

public class JL_4_reverseNum {
  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);
    System.out.println("Please input the number:");
    int n = sc.nextInt();
    int orignal = n;
    int rev = 0;

    while (n > 0) {
      int x = n % 10;
      rev = (rev * 10) + x;
      n = n / 10;
    }
    System.out.println("The reverse of " + orignal + " is " + rev);
    sc.close();
  }
}
