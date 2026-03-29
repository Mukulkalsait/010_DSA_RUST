import java.util.*;

public class JFO_1_2_PrimeCheck {

  public static boolean checkPrime(int a) {
    if (a == 2)
      return true;
    for (int i = 2; i <= Math.sqrt(a); i++) {
      if (a % i == 0) {
        return false;
      }
    }
    return true;
  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    System.out.println("please inter the Initail Number:");
    int startNum = sc.nextInt();
    System.out.println("please inter the Final Number:");
    int endNum = sc.nextInt();

    // TEST: test
    // System.out.println(checkPrime(startNum));
    //
    // TAG: i1 for making sure only "10" numbers will occupy one line.
    int i1 = 0;

    do {
      if (checkPrime(startNum) == true) {
        if (i1 % 10 == 0) {
          System.out.println(startNum);
        } else {
          System.out.print(startNum + " ");
        }
        // TAG: i1
        i1++;
      }
      startNum++;
    } while (startNum <= endNum);

    sc.close();
  }
}
