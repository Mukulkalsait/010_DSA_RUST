import java.util.*;

public class JL_Pat_1_lines {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("please input number of lines:");
    int line = sc.nextInt();

    for (int i = 0; i < line; i++) {
      System.out.println("****");
    }
    sc.close();
  }
}
