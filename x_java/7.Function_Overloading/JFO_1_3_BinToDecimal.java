import java.util.*;

public class JFO_1_3_BinToDecimal {

  public static int binToDec(int binNum) {
    int root = 0;
    int num = 0;
    do {
      int lastDig = binNum % 10;
      num = num + (lastDig * (int) Math.pow(2, root));
      root++;
      binNum = binNum / 10;
    } while (binNum > 0);
    return num;
  }

  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);
    System.out.println("please input the binary number in the form of 1's and 0's :");
    int binNum = sc.nextInt();
    System.out.println(binToDec(binNum));

    sc.close();
  }
}
