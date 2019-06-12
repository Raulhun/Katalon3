import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.By as By
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.annotation.Keyword as Keyword
import org.openqa.selenium.WebDriver as WebDriver
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import org.openqa.selenium.Keys as Keys

WebUI.click(findTestObject('Elojegyzes/btn_Elojeyzes_sidebar'))

WebUI.click(findTestObject('Elojegyzes/btn_Naptar_nezet_Elojegyzes'))

WebUI.waitForElementVisible(findTestObject('Elojegyzes/select_Szakrendeles'), 30)

WebUI.deselectAllOption(findTestObject('Elojegyzes/select_Szakrendeles'))

WebUI.delay(1)

WebUI.selectOptionByIndex(findTestObject('Elojegyzes/select_Szakrendeles'), 0)

WebUI.delay(1)

WebUI.verifyOptionSelectedByIndex(findTestObject('Elojegyzes/select_Szakrendeles'), 0, 60)

WebUI.delay(2)

WebUI.click(findTestObject('Elojegyzes/tab_Rendelesi_napok_kezelese'))

//Tábla kezelése
WebDriver driver = DriverFactory.getWebDriver()

WebElement Webtable = driver.findElement(By.id('elojegyzes-heti'))

List<WebElement> rows_table = Webtable.findElements(By.tagName('tr'))

'To calculate no of rows In table'
int rows_count = rows_table.size()

KeywordUtil.markWarning('Ennyi sor van: ' + rows_count)

'Loop will execute for all the rows of the table'
Loop:
 
  'To locate columns(cells) of that specific row'
  List < WebElement > Columns_row = rows_table.get(0).findElements(By.tagName('th'))
  
  'To calculate no of columns(cells) In that specific row'
  int columns_count = Columns_row.size()
  
  KeywordUtil.markWarning(('Number of cells In Row 1 are ') + columns_count)
  
  'Loop will execute till the last cell of that specific row'
  for (int column = 0; column < columns_count; column++) {
   
  'It will retrieve text from each cell'
   String celltext = Columns_row.get(column).getText()
   KeywordUtil.markWarning(((('Cell Value Of row number and column number ') + column) + ' Is ') + celltext)
   
   'Checking if Cell text is matching with the expected value'
   
   if (celltext == ExpectedValue) {
	   
	'Getting the Country Name if cell text i.e Company name matches with Expected value'
	KeywordUtil.markWarning('Text present in row number 3 is: ' + Columns_row.get(2).getText())
	
	'After getting the Expected value from Table we will Terminate the loop'
	break Loop;
   }
  }
 

