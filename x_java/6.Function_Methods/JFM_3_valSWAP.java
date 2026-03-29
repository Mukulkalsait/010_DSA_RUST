import java.util.*;

public class JFM_3_valSWAP {

  // TODO: CALL by value:
  // ___________________
  // in java all the PARAMETERS that get passed inside fun()
  // are "Pass by value" meen a COPY is created in stack therefore
  // if we changed the value in that function and in main function
  // billow the fun() exicution we try to print value of that var
  // java will print orignal value becouse the changed value is collected
  // with stack after exicution.

  // example:
  public static void changeVal(int a, int b) {
    int temp = a;
    a = b;
    b = temp;

    System.out.println("value of A and B respectively in side function  " + a + " " + b);
  }

  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);

    System.out.println("Inter A:");
    int a = sc.nextInt();
    System.out.println("Inter B:");
    int b = sc.nextInt();

    changeVal(a, b);

    System.out.println("value of A and B respectively in MAIN function " + a + " " + b);
    System.out.println(
        "The resualt shows how jawa uses 'Pass by value' to copy the variables in other method/function/stack");
    sc.close();
  }
}
