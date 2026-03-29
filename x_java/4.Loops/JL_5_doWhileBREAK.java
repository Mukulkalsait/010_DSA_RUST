import java.util.*;

public class JL_5_doWhileBREAK {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    do {
      System.out.println("Inter Value:");
      int num = sc.nextInt();
      if (num % 10 == 0) {
        break;
      }
      System.out.println(num);
    } while (true);
    sc.close();
  }
}
